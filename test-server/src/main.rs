use clap::{Parser, Subcommand};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, UdpSocket};
use tokio::time;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Parser)]
#[command(name = "test-server")]
#[command(about = "TCP/UDP test message generator for wasmCloud provider testing")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send messages via TCP
    Tcp {
        /// Target host
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,

        /// Target port
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Number of messages to send (0 for infinite)
        #[arg(short, long, default_value = "10")]
        count: usize,

        /// Interval between messages in milliseconds
        #[arg(short, long, default_value = "1000")]
        interval: u64,

        /// Message prefix
        #[arg(short, long, default_value = "TCP Message")]
        message: String,
    },
    /// Send messages via UDP
    Udp {
        /// Target host
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,

        /// Target port
        #[arg(short, long, default_value = "8081")]
        port: u16,

        /// Number of messages to send (0 for infinite)
        #[arg(short, long, default_value = "10")]
        count: usize,

        /// Interval between messages in milliseconds
        #[arg(short, long, default_value = "1000")]
        interval: u64,

        /// Message prefix
        #[arg(short, long, default_value = "UDP Message")]
        message: String,
    },
    /// Send messages via both TCP and UDP
    Both {
        /// Target host
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,

        /// TCP port
        #[arg(long, default_value = "8080")]
        tcp_port: u16,

        /// UDP port
        #[arg(long, default_value = "8081")]
        udp_port: u16,

        /// Number of messages to send (0 for infinite)
        #[arg(short, long, default_value = "10")]
        count: usize,

        /// Interval between messages in milliseconds
        #[arg(short, long, default_value = "1000")]
        interval: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tcp {
            host,
            port,
            count,
            interval,
            message,
        } => {
            send_tcp_messages(&host, port, count, interval, &message).await?;
        }
        Commands::Udp {
            host,
            port,
            count,
            interval,
            message,
        } => {
            send_udp_messages(&host, port, count, interval, &message).await?;
        }
        Commands::Both {
            host,
            tcp_port,
            udp_port,
            count,
            interval,
        } => {
            let host_tcp = host.clone();
            let host_udp = host.clone();
            let tcp_task = tokio::spawn(async move {
                send_tcp_messages(&host_tcp, tcp_port, count, interval, "TCP Message").await
            });
            let udp_task = tokio::spawn(async move {
                send_udp_messages(&host_udp, udp_port, count, interval, "UDP Message").await
            });

            let (tcp_result, udp_result) = tokio::join!(tcp_task, udp_task);
            tcp_result.unwrap()?;
            udp_result.unwrap()?;
        }
    }

    Ok(())
}

async fn send_tcp_messages(
    host: &str,
    port: u16,
    count: usize,
    interval: u64,
    message_prefix: &str,
) -> Result<()> {
    let addr = format!("{}:{}", host, port);
    println!("Connecting to TCP {}...", addr);

    let mut stream = TcpStream::connect(&addr).await?;
    println!("Connected to TCP {}", addr);

    let mut counter = 0;
    let infinite = count == 0;

    while infinite || counter < count {
        counter += 1;
        let message = format!("{} #{}\n", message_prefix, counter);
        stream.write_all(message.as_bytes()).await?;
        println!("Sent TCP: {}", message.trim());

        if !infinite && counter >= count {
            break;
        }

        time::sleep(Duration::from_millis(interval)).await;
    }

    println!("TCP sending complete");
    Ok(())
}

async fn send_udp_messages(
    host: &str,
    port: u16,
    count: usize,
    interval: u64,
    message_prefix: &str,
) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let addr = format!("{}:{}", host, port);
    println!("Sending UDP messages to {}...", addr);

    let mut counter = 0;
    let infinite = count == 0;

    while infinite || counter < count {
        counter += 1;
        let message = format!("{} #{}\n", message_prefix, counter);
        socket.send_to(message.as_bytes(), &addr).await?;
        println!("Sent UDP: {}", message.trim());

        if !infinite && counter >= count {
            break;
        }

        time::sleep(Duration::from_millis(interval)).await;
    }

    println!("UDP sending complete");
    Ok(())
}
