use tcp_udp_stream_provider::TcpUdpStreamProvider;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("tcp_udp_stream_provider=info")
        .init();

    // Run the provider
    TcpUdpStreamProvider::run().await
}
