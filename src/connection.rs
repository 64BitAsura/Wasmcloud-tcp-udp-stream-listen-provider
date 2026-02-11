use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

use crate::handler::MessageHandler;

/// Type of connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionType {
    Tcp,
    Udp,
}

/// Connection identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionId {
    conn_type: ConnectionType,
    host: String,
    port: u16,
}

impl ConnectionId {
    fn new(conn_type: ConnectionType, host: String, port: u16) -> Self {
        Self {
            conn_type,
            host,
            port,
        }
    }
}

impl std::fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}://{}:{}", self.conn_type, self.host, self.port)
    }
}

/// Active connection with its task handle
struct ActiveConnection {
    id: ConnectionId,
    task: JoinHandle<()>,
}

/// Manages TCP and UDP connections
pub struct ConnectionManager {
    connections: HashMap<ConnectionId, ActiveConnection>,
    shutdown_signal: Arc<Mutex<bool>>,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            shutdown_signal: Arc::new(Mutex::new(false)),
        }
    }

    /// Add a new connection
    pub async fn add_connection(
        &mut self,
        conn_type: ConnectionType,
        host: String,
        port: u16,
        message_handler: Arc<MessageHandler>,
    ) -> Result<()> {
        let id = ConnectionId::new(conn_type, host.clone(), port);

        if self.connections.contains_key(&id) {
            warn!("Connection {} already exists, skipping", id.to_string());
            return Ok(());
        }

        let task = match conn_type {
            ConnectionType::Tcp => {
                let h = host.clone();
                let p = port;
                tokio::spawn(async move {
                    if let Err(e) = Self::tcp_connection_loop(h, p, message_handler).await {
                        error!("TCP connection error: {}", e);
                    }
                })
            }
            ConnectionType::Udp => {
                let h = host.clone();
                let p = port;
                tokio::spawn(async move {
                    if let Err(e) = Self::udp_connection_loop(h, p, message_handler).await {
                        error!("UDP connection error: {}", e);
                    }
                })
            }
        };

        self.connections.insert(
            id.clone(),
            ActiveConnection {
                id: id.clone(),
                task,
            },
        );

        info!("Connection {} started", id);
        Ok(())
    }

    /// TCP connection loop - receives messages continuously
    async fn tcp_connection_loop(
        host: String,
        port: u16,
        message_handler: Arc<MessageHandler>,
    ) -> Result<()> {
        info!("Connecting to TCP {}:{}", host, port);

        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(&addr)
            .await
            .context(format!("Failed to connect to {}", addr))?;

        info!("TCP connection established to {}", addr);

        let mut buffer = vec![0u8; 4096];

        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    warn!("TCP connection closed by remote: {}", addr);
                    break;
                }
                Ok(n) => {
                    debug!("Received {} bytes from TCP {}", n, addr);

                    // Convert to ASCII string
                    let message = String::from_utf8_lossy(&buffer[..n]).to_string();

                    // Forward to message handler
                    if let Err(e) = message_handler.handle_message(&addr, message).await {
                        error!("Failed to handle TCP message: {}", e);
                    }
                }
                Err(e) => {
                    error!("TCP read error from {}: {}", addr, e);
                    break;
                }
            }
        }

        info!("TCP connection loop ended for {}", addr);
        Ok(())
    }

    /// UDP connection loop - receives messages continuously
    async fn udp_connection_loop(
        host: String,
        port: u16,
        message_handler: Arc<MessageHandler>,
    ) -> Result<()> {
        info!("Setting up UDP connection to {}:{}", host, port);

        // Bind to local address for receiving
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .context("Failed to bind UDP socket")?;

        let addr = format!("{}:{}", host, port);
        socket
            .connect(&addr)
            .await
            .context(format!("Failed to connect UDP socket to {}", addr))?;

        info!("UDP connection established to {}", addr);

        let mut buffer = vec![0u8; 4096];

        loop {
            match socket.recv(&mut buffer).await {
                Ok(n) => {
                    debug!("Received {} bytes from UDP {}", n, addr);

                    // Convert to ASCII string
                    let message = String::from_utf8_lossy(&buffer[..n]).to_string();

                    // Forward to message handler
                    if let Err(e) = message_handler.handle_message(&addr, message).await {
                        error!("Failed to handle UDP message: {}", e);
                    }
                }
                Err(e) => {
                    error!("UDP recv error from {}: {}", addr, e);
                    break;
                }
            }
        }

        info!("UDP connection loop ended for {}", addr);
        Ok(())
    }

    /// Stop all connections
    pub async fn stop_all(&mut self) -> Result<()> {
        info!("Stopping all connections");

        *self.shutdown_signal.lock().await = true;

        // Abort all tasks
        for (_, conn) in self.connections.drain() {
            info!("Stopping connection {}", conn.id);
            conn.task.abort();
        }

        info!("All connections stopped");
        Ok(())
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_id() {
        let id = ConnectionId::new(ConnectionType::Tcp, "localhost".to_string(), 8080);
        assert_eq!(id.conn_type, ConnectionType::Tcp);
        assert_eq!(id.host, "localhost");
        assert_eq!(id.port, 8080);
        assert_eq!(id.to_string(), "Tcp://localhost:8080");
    }

    #[tokio::test]
    async fn test_connection_manager_creation() {
        let manager = ConnectionManager::new();
        assert_eq!(manager.connections.len(), 0);
    }
}
