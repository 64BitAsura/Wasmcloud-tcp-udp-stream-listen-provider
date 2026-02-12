//! TCP/UDP stream listen provider for wasmcloud:messaging.
//!
//! This provider acts as a unidirectional TCP/UDP ASCII message stream client:
//! it connects to a remote TCP/UDP server and forwards received
//! ASCII messages to wasmCloud components via the NATS mesh using the
//! `wasmcloud:messaging/handler.handle-message` callback.
//!
//! Reply-back feature is deferred to a future release.

mod connection;
mod stream;

use stream::TcpUdpStreamProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    TcpUdpStreamProvider::run().await?;
    eprintln!("TCP/UDP stream listen provider exiting");
    Ok(())
}
