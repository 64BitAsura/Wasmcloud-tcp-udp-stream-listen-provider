use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use wasmcloud_provider_sdk::{core::HostData, LinkConfig, LinkDeleteInfo, Provider};

mod connection;
mod handler;
mod nats;

use connection::{ConnectionManager, ConnectionType};
use handler::MessageHandler;

/// Configuration for TCP connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConfig {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_reconnect_delay")]
    pub reconnect_delay_ms: u64,
}

/// Configuration for UDP connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpConfig {
    pub host: String,
    pub port: u16,
}

fn default_reconnect_delay() -> u64 {
    5000
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    #[serde(default)]
    pub tcp: Vec<TcpConfig>,
    #[serde(default)]
    pub udp: Vec<UdpConfig>,
}

/// Main provider implementation
#[derive(Clone)]
pub struct TcpUdpStreamProvider {
    /// Host data from WasmCloud
    #[allow(dead_code)]
    host_data: Arc<HostData>,
    /// Connection manager for TCP/UDP connections
    connection_manager: Arc<RwLock<ConnectionManager>>,
    /// Message handler for processing incoming messages
    message_handler: Arc<MessageHandler>,
    /// Active link configurations
    links: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
}

impl TcpUdpStreamProvider {
    /// Create a new provider instance
    pub async fn new(host_data: HostData) -> Result<Self> {
        info!("Initializing TCP/UDP Stream Provider");

        let connection_manager = ConnectionManager::new();
        let message_handler = MessageHandler::new(host_data.lattice_rpc_url.clone());

        Ok(Self {
            host_data: Arc::new(host_data),
            connection_manager: Arc::new(RwLock::new(connection_manager)),
            message_handler: Arc::new(message_handler),
            links: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Start connections based on configuration
    async fn start_connections(&self, config: ProviderConfig) -> Result<()> {
        info!("Starting connections from configuration");

        let mut manager = self.connection_manager.write().await;

        // Start TCP connections
        for tcp_config in config.tcp {
            info!(
                "Starting TCP connection to {}:{}",
                tcp_config.host, tcp_config.port
            );
            manager
                .add_connection(
                    ConnectionType::Tcp,
                    tcp_config.host,
                    tcp_config.port,
                    self.message_handler.clone(),
                )
                .await?;
        }

        // Start UDP connections
        for udp_config in config.udp {
            info!(
                "Starting UDP connection to {}:{}",
                udp_config.host, udp_config.port
            );
            manager
                .add_connection(
                    ConnectionType::Udp,
                    udp_config.host,
                    udp_config.port,
                    self.message_handler.clone(),
                )
                .await?;
        }

        Ok(())
    }
}

impl Provider for TcpUdpStreamProvider {
    async fn receive_link_config_as_target(
        &self,
        LinkConfig {
            source_id, config, ..
        }: LinkConfig<'_>,
    ) -> Result<()> {
        info!("Received link configuration from source: {}", source_id);

        // Parse provider configuration from link values
        let provider_config: ProviderConfig = if !config.is_empty() {
            serde_json::from_str(&serde_json::to_string(&config)?)?
        } else {
            warn!("No configuration provided in link, using defaults");
            ProviderConfig::default()
        };

        // Store link configuration
        self.links
            .write()
            .await
            .insert(source_id.to_string(), config.to_owned());

        // Start connections
        if let Err(e) = self.start_connections(provider_config).await {
            error!("Failed to start connections: {}", e);
            return Err(e);
        }

        info!("Link established successfully");
        Ok(())
    }

    async fn delete_link_as_target(&self, info: impl LinkDeleteInfo) -> Result<()> {
        let source_id = info.get_source_id();
        info!("Deleting link for source: {}", source_id);

        self.links.write().await.remove(source_id);

        // Stop all connections (simplified - in production would track per-link)
        let mut manager = self.connection_manager.write().await;
        manager.stop_all().await?;

        info!("Link deleted successfully");
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        info!("Shutting down TCP/UDP Stream Provider");

        // Stop all connections
        let mut manager = self.connection_manager.write().await;
        manager.stop_all().await?;

        // Shutdown message handler
        self.message_handler.shutdown().await?;

        info!("Provider shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let json = r#"{"tcp":[{"host":"localhost","port":8080}],"udp":[{"host":"localhost","port":9090}]}"#;
        let config: Result<ProviderConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.tcp.len(), 1);
        assert_eq!(config.udp.len(), 1);
        assert_eq!(config.tcp[0].host, "localhost");
        assert_eq!(config.tcp[0].port, 8080);
    }

    #[test]
    fn test_default_config() {
        let config = ProviderConfig::default();
        assert_eq!(config.tcp.len(), 0);
        assert_eq!(config.udp.len(), 0);
    }

    #[test]
    fn test_tcp_config_with_defaults() {
        let json = r#"{"tcp":[{"host":"example.com","port":8080}]}"#;
        let config: Result<ProviderConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.tcp[0].reconnect_delay_ms, 5000);
    }
}
