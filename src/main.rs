use anyhow::Result;
use tracing::info;
use wasmcloud_provider_sdk::{load_host_data, run_provider};
use wasmcloud_provider_tcp_udp_stream::TcpUdpStreamProvider;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("TCP/UDP Stream Provider starting");

    // Load host data from environment
    let host_data = load_host_data()?;

    // Create provider instance
    let provider = TcpUdpStreamProvider::new(host_data.clone()).await?;

    // Run the provider
    let _result = run_provider(provider, "tcp-udp-stream-provider").await?;

    info!("TCP/UDP Stream Provider stopped");
    Ok(())
}
