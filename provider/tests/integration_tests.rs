use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time::sleep;

/// Integration tests for TCP connection
#[tokio::test]
async fn test_tcp_server_connection() {
    // Start a mock TCP server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Spawn server task
    let server_task = tokio::spawn(async move {
        if let Ok((mut socket, _)) = listener.accept().await {
            // Send a test message
            let _ = socket.write_all(b"Hello from TCP server\n").await;
            sleep(Duration::from_millis(100)).await;
        }
    });

    // Give server time to start
    sleep(Duration::from_millis(100)).await;

    // Connect to the server
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();

    // Read response
    use tokio::io::AsyncReadExt;
    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&mut buffer).await.unwrap();

    let message = String::from_utf8_lossy(&buffer[..n]);
    assert_eq!(message, "Hello from TCP server\n");

    // Cleanup
    server_task.abort();
}

/// Integration tests for UDP connection
#[tokio::test]
async fn test_udp_server_connection() {
    use tokio::net::UdpSocket;

    // Create a mock UDP server
    let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let server_addr = server.local_addr().unwrap();

    // Create a client socket
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    // Send a message to the server
    client
        .send_to(b"Hello from UDP client", server_addr)
        .await
        .unwrap();

    // Server receives the message
    let mut buffer = vec![0u8; 1024];
    let (n, _) = server.recv_from(&mut buffer).await.unwrap();

    let message = String::from_utf8_lossy(&buffer[..n]);
    assert_eq!(message, "Hello from UDP client");
}

/// Test configuration parsing
#[test]
fn test_provider_configuration() {
    use tcp_udp_stream_provider::ProviderConfig;

    let json = r#"{
        "tcp": [
            {"host": "localhost", "port": 8080, "reconnect_delay_ms": 3000},
            {"host": "example.com", "port": 9090}
        ],
        "udp": [
            {"host": "localhost", "port": 7070}
        ]
    }"#;

    let config: ProviderConfig = serde_json::from_str(json).unwrap();

    assert_eq!(config.tcp.len(), 2);
    assert_eq!(config.udp.len(), 1);

    assert_eq!(config.tcp[0].host, "localhost");
    assert_eq!(config.tcp[0].port, 8080);
    assert_eq!(config.tcp[0].reconnect_delay_ms, 3000);

    assert_eq!(config.tcp[1].reconnect_delay_ms, 5000); // default
}
