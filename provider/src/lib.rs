use anyhow::Result;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, UdpSocket};
use tracing::{debug, error, info};

const DEFAULT_TCP_PORT: u16 = 8080;
const DEFAULT_UDP_PORT: u16 = 8081;
const MAX_MESSAGE_SIZE: usize = 65536; // 64KB

pub struct TcpUdpStreamProvider;

impl TcpUdpStreamProvider {
    pub fn new() -> Self {
        Self
    }

    pub async fn start_tcp_listener(port: u16) -> Result<()> {
        let addr = format!("0.0.0.0:{}", port);
        info!("Starting TCP listener on {}", addr);
        let listener = TcpListener::bind(&addr).await?;

        tokio::spawn(async move {
            info!("TCP listener running on {}", addr);
            loop {
                match listener.accept().await {
                    Ok((mut socket, client_addr)) => {
                        info!("Accepted TCP connection from {}", client_addr);
                        
                        tokio::spawn(async move {
                            let mut buffer = vec![0u8; MAX_MESSAGE_SIZE];
                            loop {
                                match socket.read(&mut buffer).await {
                                    Ok(0) => {
                                        debug!("TCP connection closed by {}", client_addr);
                                        break;
                                    }
                                    Ok(n) => {
                                        let message = String::from_utf8_lossy(&buffer[..n]);
                                        info!(
                                            "Received TCP message from {}: {}",
                                            client_addr,
                                            message.trim()
                                        );

                                        // Process each line as a separate message
                                        for line in message.lines() {
                                            if !line.trim().is_empty() {
                                                info!("Processing TCP line: {}", line.trim());
                                                // In a real implementation, this would forward to wasmCloud component
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Error reading from TCP socket: {}", e);
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept TCP connection: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn start_udp_listener(port: u16) -> Result<()> {
        let addr = format!("0.0.0.0:{}", port);
        info!("Starting UDP listener on {}", addr);
        let socket = Arc::new(UdpSocket::bind(&addr).await?);

        tokio::spawn(async move {
            info!("UDP listener running on {}", addr);
            let mut buffer = vec![0u8; MAX_MESSAGE_SIZE];
            loop {
                match socket.recv_from(&mut buffer).await {
                    Ok((n, client_addr)) => {
                        let message = String::from_utf8_lossy(&buffer[..n]);
                        info!("Received UDP message from {}: {}", client_addr, message.trim());

                        // Process each line as a separate message
                        for line in message.lines() {
                            if !line.trim().is_empty() {
                                info!("Processing UDP line: {}", line.trim());
                                // In a real implementation, this would forward to wasmCloud component
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error receiving UDP packet: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn run() -> Result<()> {
        info!("Starting TCP/UDP Stream Provider");
        
        // Start both listeners
        Self::start_tcp_listener(DEFAULT_TCP_PORT).await?;
        Self::start_udp_listener(DEFAULT_UDP_PORT).await?;

        info!("TCP/UDP Stream Provider started successfully");
        info!("Listening on TCP port {} and UDP port {}", DEFAULT_TCP_PORT, DEFAULT_UDP_PORT);

        // Keep running forever
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let _provider = TcpUdpStreamProvider::new();
    }
}
