//! TCP/UDP stream listen provider for wasmCloud
//!
//! This provider connects to remote TCP/UDP servers and forwards received messages
//! to wasmCloud components via wRPC. It implements unidirectional communication
//! (receiving only) with per-component stream management.

mod config;
mod provider;
mod stream;

use provider::TcpUdpStreamProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    TcpUdpStreamProvider::run().await?;
    eprintln!("TCP/UDP stream provider exiting");
    Ok(())
}
