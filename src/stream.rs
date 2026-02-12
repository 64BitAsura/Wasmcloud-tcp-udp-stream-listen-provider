use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpStream, UdpSocket};
use tracing::{debug, error, info};

use crate::config::{ConnectionConfig, StreamProtocol};

/// TCP/UDP stream client handler
pub struct StreamClient {
    config: ConnectionConfig,
}

impl StreamClient {
    /// Create a new stream client
    pub fn new(config: ConnectionConfig) -> Self {
        Self { config }
    }

    /// Connect to the remote server and start receiving messages.
    ///
    /// Calls `message_handler` for each received line (TCP) or datagram (UDP).
    /// The `shutdown_rx` is used to signal the client to stop reading.
    pub async fn run<F>(
        &self,
        mut message_handler: F,
        mut shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    ) -> anyhow::Result<()>
    where
        F: FnMut(Vec<u8>) -> anyhow::Result<()> + Send,
    {
        match self.config.protocol {
            StreamProtocol::Tcp => self.run_tcp(&mut message_handler, &mut shutdown_rx).await,
            StreamProtocol::Udp => self.run_udp(&mut message_handler, &mut shutdown_rx).await,
        }
    }

    /// Connect to a TCP server and read line-delimited ASCII messages
    async fn run_tcp<F>(
        &self,
        message_handler: &mut F,
        shutdown_rx: &mut tokio::sync::oneshot::Receiver<()>,
    ) -> anyhow::Result<()>
    where
        F: FnMut(Vec<u8>) -> anyhow::Result<()>,
    {
        let addr = self.config.addr();
        info!(addr = %addr, "connecting TCP stream");

        let stream = TcpStream::connect(&addr).await?;
        info!(addr = %addr, "TCP stream connected");

        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                _ = &mut *shutdown_rx => {
                    info!("TCP stream shutdown signal received");
                    break;
                }
                result = lines.next_line() => {
                    match result {
                        Ok(Some(line)) => {
                            debug!(line = %line, "received TCP line");
                            message_handler(line.into_bytes())?;
                        }
                        Ok(None) => {
                            info!("TCP stream EOF");
                            break;
                        }
                        Err(e) => {
                            error!(error = %e, "TCP read error");
                            return Err(e.into());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Bind a UDP socket and receive datagrams from the remote server
    async fn run_udp<F>(
        &self,
        message_handler: &mut F,
        shutdown_rx: &mut tokio::sync::oneshot::Receiver<()>,
    ) -> anyhow::Result<()>
    where
        F: FnMut(Vec<u8>) -> anyhow::Result<()>,
    {
        let addr = self.config.addr();
        info!(addr = %addr, "binding UDP socket");

        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.connect(&addr).await?;
        info!(addr = %addr, "UDP socket connected");

        let mut buf = vec![0u8; 65535];

        loop {
            tokio::select! {
                _ = &mut *shutdown_rx => {
                    info!("UDP stream shutdown signal received");
                    break;
                }
                result = socket.recv(&mut buf) => {
                    match result {
                        Ok(n) => {
                            if let Ok(line) = std::str::from_utf8(&buf[..n]) {
                                let line = line.trim_end_matches('\n').trim_end_matches('\r');
                                debug!(line = %line, "received UDP datagram");
                                message_handler(line.as_bytes().to_vec())?;
                            } else {
                                debug!("received non-UTF8 UDP datagram, skipping");
                            }
                        }
                        Err(e) => {
                            error!(error = %e, "UDP recv error");
                            return Err(e.into());
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
