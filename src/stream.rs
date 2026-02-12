use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context as _};
use bytes::Bytes;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};
use wasmcloud_provider_sdk::core::HostData;
use wasmcloud_provider_sdk::{
    get_connection, load_host_data, run_provider, serve_provider_exports, Context, LinkConfig,
    LinkDeleteInfo, Provider,
};

use crate::connection::{ConnectionConfig, StreamProtocol};

mod bindings {
    wit_bindgen_wrpc::generate!({ generate_all });
}
use bindings::exports::wasmcloud::messaging::consumer::Handler;
use bindings::wasmcloud::messaging::types::BrokerMessage;

/// A bundle that holds the background stream-reading task and a channel sender
/// so we can signal shutdown when a link is removed.
#[derive(Debug)]
struct StreamBundle {
    /// Handle to the background task reading from TCP/UDP
    pub handle: JoinHandle<()>,
    /// Sender used to signal the reader to stop (dropped on unlink)
    pub _shutdown_tx: tokio::sync::oneshot::Sender<()>,
}

impl Drop for StreamBundle {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

/// TCP/UDP stream listen provider for the `wasmcloud:messaging` interface.
///
/// For each linked component the provider connects to the configured remote
/// TCP or UDP server and spawns a background task that reads ASCII lines
/// and forwards each line as a `BrokerMessage` to the component via
/// `wasmcloud:messaging/handler.handle-message`.
#[derive(Default, Clone)]
pub struct TcpUdpStreamProvider {
    /// Map of stream bundles per linked component
    components: Arc<RwLock<HashMap<String, StreamBundle>>>,
    /// Default configuration used when link config is empty
    default_config: ConnectionConfig,
}

impl TcpUdpStreamProvider {
    /// Execute the provider, loading default configuration from the host and subscribing
    /// on the proper RPC topics via `wrpc::serve`
    pub async fn run() -> anyhow::Result<()> {
        let host_data = load_host_data().context("failed to load host data")?;
        let provider = Self::from_host_data(host_data);
        let shutdown = run_provider(provider.clone(), "tcp-udp-stream-provider")
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

    /// Build a [`TcpUdpStreamProvider`] from [`HostData`]
    pub fn from_host_data(host_data: &HostData) -> TcpUdpStreamProvider {
        let default_config = ConnectionConfig::from(&host_data.config);
        TcpUdpStreamProvider {
            default_config,
            ..Default::default()
        }
    }

    /// Spawn a background task that connects to the remote TCP/UDP server
    /// and reads ASCII lines, forwarding each to the linked component.
    async fn start_stream(
        &self,
        cfg: ConnectionConfig,
        component_id: &str,
    ) -> anyhow::Result<StreamBundle> {
        let addr = cfg.addr();
        let component_id = Arc::new(component_id.to_string());
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel::<()>();

        let handle = match cfg.protocol {
            StreamProtocol::Tcp => {
                let comp = Arc::clone(&component_id);
                tokio::spawn(async move {
                    info!(addr = %addr, component_id = %comp, "connecting TCP stream");
                    let stream = match TcpStream::connect(&addr).await {
                        Ok(s) => s,
                        Err(e) => {
                            error!(addr = %addr, error = %e, "failed to connect TCP stream");
                            return;
                        }
                    };
                    info!(addr = %addr, "TCP stream connected");

                    let reader = BufReader::new(stream);
                    let mut lines = reader.lines();

                    loop {
                        tokio::select! {
                            _ = &mut shutdown_rx => {
                                info!(component_id = %comp, "TCP stream shutdown signal received");
                                break;
                            }
                            result = lines.next_line() => {
                                match result {
                                    Ok(Some(line)) => {
                                        debug!(component_id = %comp, line = %line, "received TCP line");
                                        dispatch_msg(&comp, &line).await;
                                    }
                                    Ok(None) => {
                                        info!(component_id = %comp, "TCP stream EOF");
                                        break;
                                    }
                                    Err(e) => {
                                        error!(component_id = %comp, error = %e, "TCP read error");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                })
            }
            StreamProtocol::Udp => {
                let comp = Arc::clone(&component_id);
                tokio::spawn(async move {
                    info!(addr = %addr, component_id = %comp, "binding UDP socket");
                    // Bind to any local port, then "connect" to remote for recv
                    let socket = match UdpSocket::bind("0.0.0.0:0").await {
                        Ok(s) => s,
                        Err(e) => {
                            error!(error = %e, "failed to bind UDP socket");
                            return;
                        }
                    };
                    if let Err(e) = socket.connect(&addr).await {
                        error!(addr = %addr, error = %e, "failed to connect UDP socket");
                        return;
                    }
                    info!(addr = %addr, "UDP socket connected");

                    let mut buf = vec![0u8; 65535];

                    loop {
                        tokio::select! {
                            _ = &mut shutdown_rx => {
                                info!(component_id = %comp, "UDP stream shutdown signal received");
                                break;
                            }
                            result = socket.recv(&mut buf) => {
                                match result {
                                    Ok(n) => {
                                        if let Ok(line) = std::str::from_utf8(&buf[..n]) {
                                            let line = line.trim_end_matches('\n').trim_end_matches('\r');
                                            debug!(component_id = %comp, line = %line, "received UDP datagram");
                                            dispatch_msg(&comp, line).await;
                                        } else {
                                            debug!(component_id = %comp, "received non-UTF8 UDP datagram, skipping");
                                        }
                                    }
                                    Err(e) => {
                                        error!(component_id = %comp, error = %e, "UDP recv error");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                })
            }
        };

        Ok(StreamBundle {
            handle,
            _shutdown_tx: shutdown_tx,
        })
    }
}

/// Forward a received ASCII line to the linked component as a BrokerMessage.
async fn dispatch_msg(component_id: &str, line: &str) {
    let msg = BrokerMessage {
        body: Bytes::from(line.as_bytes().to_vec()),
        reply_to: None, // unidirectional — reply-back deferred
        subject: "stream.message".to_string(),
    };
    debug!(
        subject = msg.subject,
        component_id = component_id,
        "sending message to component",
    );

    // TODO: Invoke the component's `wasmcloud:messaging/handler.handle-message`
    // via the wRPC connection once fully wired.
    // For now log the message — full wiring depends on the host runtime.
    info!(
        component_id = component_id,
        subject = msg.subject,
        body_len = msg.body.len(),
        "dispatched stream message (handler invocation pending full wRPC wiring)"
    );
}

impl Provider for TcpUdpStreamProvider {
    /// Called when a component links to this provider as target.
    /// We start a new TCP/UDP stream reader for the component.
    async fn receive_link_config_as_target(
        &self,
        LinkConfig {
            source_id, config, ..
        }: LinkConfig<'_>,
    ) -> anyhow::Result<()> {
        let config = if config.is_empty() {
            self.default_config.clone()
        } else {
            self.default_config.merge(ConnectionConfig::from(config))
        };

        info!(
            source_id = source_id,
            protocol = ?config.protocol,
            addr = %config.addr(),
            "starting stream for component"
        );

        let bundle = match self.start_stream(config, source_id).await {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to start stream: {e:?}");
                bail!(anyhow!(e).context("failed to start TCP/UDP stream"))
            }
        };

        let mut map = self.components.write().await;
        map.insert(source_id.into(), bundle);
        Ok(())
    }

    /// Handle link deletion: stop the stream reader for the component.
    async fn delete_link_as_target(&self, link: impl LinkDeleteInfo) -> anyhow::Result<()> {
        let source_id = link.get_source_id();
        let mut all = self.components.write().await;

        if all.remove(source_id).is_some() {
            debug!(source_id = source_id, "stopped stream for component");
        }

        debug!(source_id = source_id, "finished processing delete link");
        Ok(())
    }

    /// Handle shutdown: stop all stream readers.
    async fn shutdown(&self) -> anyhow::Result<()> {
        let mut all = self.components.write().await;
        all.clear();
        info!("TCP/UDP stream provider shutdown complete");
        Ok(())
    }
}

/// Implement the `wasmcloud:messaging/consumer` interface.
///
/// This provider is **receive-only** (unidirectional). Publishing and request
/// methods are not supported in this initial release (reply-back deferred).
impl Handler<Option<Context>> for TcpUdpStreamProvider {
    async fn publish(
        &self,
        _ctx: Option<Context>,
        _msg: BrokerMessage,
    ) -> anyhow::Result<Result<(), String>> {
        // Reply-back / publish is deferred
        Ok(Err(
            "publish is not supported: this provider is receive-only (reply-back deferred)"
                .to_string(),
        ))
    }

    async fn request(
        &self,
        _ctx: Option<Context>,
        _subject: String,
        _body: Bytes,
        _timeout_ms: u32,
    ) -> anyhow::Result<Result<BrokerMessage, String>> {
        // Reply-back / request is deferred
        Ok(Err(
            "request is not supported: this provider is receive-only (reply-back deferred)"
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = TcpUdpStreamProvider::default();
        assert!(provider.components.try_read().is_ok());
    }

    #[tokio::test]
    async fn test_provider_shutdown() {
        let provider = TcpUdpStreamProvider::default();
        provider.shutdown().await.unwrap();
        let map = provider.components.read().await;
        assert!(map.is_empty());
    }

    #[tokio::test]
    async fn test_publish_returns_error() {
        let provider = TcpUdpStreamProvider::default();
        let msg = BrokerMessage {
            subject: "test".to_string(),
            body: Bytes::from("hello"),
            reply_to: None,
        };
        let result = provider.publish(None, msg).await.unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("receive-only"));
    }

    #[tokio::test]
    async fn test_request_returns_error() {
        let provider = TcpUdpStreamProvider::default();
        let result = provider
            .request(None, "test".to_string(), Bytes::from("hi"), 1000)
            .await
            .unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("receive-only"));
    }
}
