use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context as _;
use bytes::Bytes;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use wasmcloud_provider_sdk::initialize_observability;
use wasmcloud_provider_sdk::{
    get_connection, run_provider, serve_provider_exports, Context as SdkContext,
    LinkConfig as SdkLinkConfig, LinkDeleteInfo, Provider, ProviderInitConfig,
};

use crate::config::{ConnectionConfig, ProviderConfig};
use crate::stream::StreamClient;

pub(crate) mod bindings {
    wit_bindgen_wrpc::generate!({ generate_all });
}

// Import the standard messaging interfaces from WIT
use bindings::wasmcloud::messaging::handler;
use bindings::wasmcloud::messaging::types;

/// State for a single stream connection
struct ConnectionState {
    /// Configuration for this connection
    _config: ConnectionConfig,
    /// Handle to the background stream task
    _task_handle: tokio::task::JoinHandle<()>,
    /// Shutdown signal sender — dropping this triggers stream shutdown
    _shutdown_tx: tokio::sync::oneshot::Sender<()>,
}

/// TCP/UDP stream listen provider implementation
#[derive(Default, Clone)]
pub struct TcpUdpStreamProvider {
    config: Arc<RwLock<ProviderConfig>>,
    /// All components linked to this provider (target) and their connections
    connections: Arc<RwLock<HashMap<String, ConnectionState>>>,
    /// Default configuration used when link config is empty
    default_config: ConnectionConfig,
}

impl TcpUdpStreamProvider {
    fn name() -> &'static str {
        "tcp-udp-stream-provider"
    }

    /// Execute the provider
    pub async fn run() -> anyhow::Result<()> {
        initialize_observability!(
            Self::name(),
            std::env::var_os("PROVIDER_TCP_UDP_STREAM_FLAMEGRAPH_PATH")
        );

        let provider = Self::default();
        let shutdown = run_provider(provider.clone(), Self::name())
            .await
            .context("failed to run provider")?;

        let connection = get_connection();
        serve_provider_exports(
            &connection
                .get_wrpc_client(connection.provider_key())
                .await
                .context("failed to get wrpc client")?,
            provider,
            shutdown,
            bindings::serve,
        )
        .await
    }
}

/// Implement the Provider trait for wasmCloud integration
impl Provider for TcpUdpStreamProvider {
    /// Initialize the provider
    async fn init(&self, config: impl ProviderInitConfig) -> anyhow::Result<()> {
        let provider_id = config.get_provider_id();
        let initial_config = config.get_config();
        info!(
            provider_id,
            ?initial_config,
            "initializing TCP/UDP stream provider"
        );

        // Save configuration to provider state
        *self.config.write().await = ProviderConfig::from(initial_config);

        Ok(())
    }

    /// Handle incoming link from a component (component links TO this provider).
    /// This is where we start the TCP/UDP stream client.
    async fn receive_link_config_as_target(
        &self,
        SdkLinkConfig {
            source_id, config, ..
        }: SdkLinkConfig<'_>,
    ) -> anyhow::Result<()> {
        info!("Received link configuration from component: {}", source_id);

        // Parse link configuration
        let link_config = if config.is_empty() {
            self.default_config.clone()
        } else {
            self.default_config.merge(ConnectionConfig::from(config))
        };

        info!(
            protocol = ?link_config.protocol,
            addr = %link_config.addr(),
            "Starting stream client for component: {}", source_id
        );

        // Clone what we need for the task
        let config_clone = link_config.clone();
        let source_id_clone = source_id.to_string();
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

        // Spawn stream client task
        let task_handle = tokio::spawn(async move {
            let stream_client = StreamClient::new(config_clone.clone());

            // Create message handler that forwards to the component via wRPC
            // using the standard wasmcloud:messaging interface
            let addr = config_clone.addr();
            let result = stream_client
                .run(
                    move |data| {
                        // Convert stream message to a standard broker-message
                        let message = create_broker_message(data, &addr);

                        // Spawn a task to send message to component
                        let source = source_id_clone.clone();
                        tokio::spawn(async move {
                            if let Err(e) = send_message_to_component(&source, message).await {
                                error!("Failed to send message to component {}: {}", source, e);
                            }
                        });

                        Ok(())
                    },
                    shutdown_rx,
                )
                .await;

            if let Err(e) = result {
                error!("Stream client error: {}", e);
            }
        });

        // Store connection state
        self.connections.write().await.insert(
            source_id.to_string(),
            ConnectionState {
                _config: link_config,
                _task_handle: task_handle,
                _shutdown_tx: shutdown_tx,
            },
        );

        info!("Stream connection established for component: {}", source_id);
        Ok(())
    }

    /// Handle link deletion
    async fn delete_link_as_target(&self, link: impl LinkDeleteInfo) -> anyhow::Result<()> {
        let source_id = link.get_source_id();
        info!("Deleting link with component: {}", source_id);

        // Remove connection state (task will be cancelled)
        if let Some(state) = self.connections.write().await.remove(source_id) {
            info!("Stream connection closed for component: {}", source_id);
            state._task_handle.abort();
        } else {
            warn!("No connection found for component: {}", source_id);
        }

        Ok(())
    }

    /// Handle provider shutdown
    async fn shutdown(&self) -> anyhow::Result<()> {
        info!("Shutting down TCP/UDP stream provider");

        // Clean up all connections
        let mut connections = self.connections.write().await;
        for (source_id, state) in connections.drain() {
            info!("Closing stream connection for component: {}", source_id);
            state._task_handle.abort();
        }

        info!("TCP/UDP stream provider shutdown complete");
        Ok(())
    }
}

/// Implement the `wasmcloud:messaging/consumer` interface.
///
/// This provider is **receive-only** (unidirectional). Publishing and request
/// methods are not supported in this initial release (reply-back deferred).
impl bindings::exports::wasmcloud::messaging::consumer::Handler<Option<SdkContext>>
    for TcpUdpStreamProvider
{
    async fn publish(
        &self,
        _ctx: Option<SdkContext>,
        _msg: types::BrokerMessage,
    ) -> anyhow::Result<Result<(), String>> {
        // Reply-back / publish is deferred
        Ok(Err(
            "publish is not supported: this provider is receive-only (reply-back deferred)"
                .to_string(),
        ))
    }

    async fn request(
        &self,
        _ctx: Option<SdkContext>,
        _subject: String,
        _body: Bytes,
        _timeout_ms: u32,
    ) -> anyhow::Result<Result<types::BrokerMessage, String>> {
        // Reply-back / request is deferred
        Ok(Err(
            "request is not supported: this provider is receive-only (reply-back deferred)"
                .to_string(),
        ))
    }
}

/// Create a broker-message from raw stream data.
///
/// The subject is set to "stream.<protocol>://<host:port>" so the component knows
/// which stream connection the message originated from.
/// The body contains the raw bytes of the received message.
fn create_broker_message(data: Vec<u8>, addr: &str) -> types::BrokerMessage {
    types::BrokerMessage {
        subject: format!("stream.{}", addr),
        body: data.into(),
        reply_to: None,
    }
}

/// Send message to component via wRPC using the standard messaging handler
async fn send_message_to_component(
    component_id: &str,
    message: types::BrokerMessage,
) -> anyhow::Result<()> {
    let client = wasmcloud_provider_sdk::get_connection()
        .get_wrpc_client(component_id)
        .await
        .context("failed to get wrpc client")?;

    match handler::handle_message(&client, None, &message).await {
        Ok(Ok(_)) => {
            info!("Message successfully sent to component {}", component_id);
            Ok(())
        }
        Ok(Err(e)) => {
            error!("Component {} returned error: {}", component_id, e);
            anyhow::bail!("Component error: {}", e)
        }
        Err(e) => {
            error!("Failed to call component {}: {}", component_id, e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = TcpUdpStreamProvider::default();
        assert!(provider.connections.try_read().is_ok());
    }

    #[tokio::test]
    async fn test_provider_shutdown() {
        let provider = TcpUdpStreamProvider::default();
        provider.shutdown().await.unwrap();
        let map = provider.connections.read().await;
        assert!(map.is_empty());
    }

    #[tokio::test]
    async fn test_publish_returns_error() {
        let provider = TcpUdpStreamProvider::default();
        let msg = types::BrokerMessage {
            subject: "test".to_string(),
            body: Bytes::from("hello"),
            reply_to: None,
        };
        let result = bindings::exports::wasmcloud::messaging::consumer::Handler::publish(
            &provider, None, msg,
        )
        .await
        .unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("receive-only"));
    }

    #[tokio::test]
    async fn test_request_returns_error() {
        let provider = TcpUdpStreamProvider::default();
        let result = bindings::exports::wasmcloud::messaging::consumer::Handler::request(
            &provider,
            None,
            "test".to_string(),
            Bytes::from("hi"),
            1000,
        )
        .await
        .unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("receive-only"));
    }

    #[test]
    fn test_create_broker_message() {
        let data = b"hello world".to_vec();
        let msg = create_broker_message(data.clone(), "127.0.0.1:9000");
        assert_eq!(msg.subject, "stream.127.0.0.1:9000");
        assert_eq!(msg.body.as_ref(), b"hello world");
        assert!(msg.reply_to.is_none());
    }
}
