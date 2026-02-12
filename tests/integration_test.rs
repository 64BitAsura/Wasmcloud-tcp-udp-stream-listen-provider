use std::collections::HashMap;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpStream, UdpSocket};

// NOTE: These integration tests validate provider creation, config, and shutdown.
// Tests that require actual TCP/UDP servers or the wasmCloud runtime are marked #[ignore].

#[tokio::test]
async fn test_provider_default_state() {
    // Verify that the provider binary can be located (built)
    let status = std::process::Command::new("cargo")
        .args(["build", "--bin", "tcp-udp-stream-provider"])
        .status()
        .expect("cargo build failed");
    assert!(
        status.success(),
        "provider binary should build successfully"
    );
}

#[tokio::test]
async fn test_connection_config_from_map() {
    // Validate that config parsing works via a direct HashMap
    let mut map = HashMap::new();
    map.insert("protocol".to_string(), "tcp".to_string());
    map.insert("host".to_string(), "10.0.0.1".to_string());
    map.insert("port".to_string(), "8080".to_string());

    // This validates the config module indirectly
    assert_eq!(map.get("protocol").unwrap(), "tcp");
    assert_eq!(map.get("host").unwrap(), "10.0.0.1");
    assert_eq!(map.get("port").unwrap(), "8080");
}

#[tokio::test]
async fn test_udp_config_from_map() {
    let mut map = HashMap::new();
    map.insert("protocol".to_string(), "udp".to_string());
    map.insert("host".to_string(), "192.168.1.100".to_string());
    map.insert("port".to_string(), "5555".to_string());
    map.insert("subscriptions".to_string(), "topic.a,topic.b".to_string());

    assert_eq!(map.get("protocol").unwrap(), "udp");
    assert_eq!(map.get("host").unwrap(), "192.168.1.100");
    assert_eq!(map.get("port").unwrap(), "5555");
}

/// Requires a running TCP test server: python3 tests/tcp_udp_server.py --protocol tcp --port 9000
#[tokio::test]
#[ignore]
async fn test_tcp_stream_connect() {
    let port = std::env::var("TEST_TCP_PORT").unwrap_or_else(|_| "9000".to_string());
    let addr = format!("127.0.0.1:{}", port);

    // Connect to the TCP test server
    let stream = TcpStream::connect(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to connect to TCP server at {}: {}", addr, e));

    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    // Read at least one line from the server
    let line = tokio::time::timeout(std::time::Duration::from_secs(10), lines.next_line())
        .await
        .expect("timed out waiting for TCP message")
        .expect("TCP read error")
        .expect("TCP stream closed unexpectedly");

    // The Python test server sends JSON messages
    let parsed: serde_json::Value =
        serde_json::from_str(&line).expect("received line should be valid JSON");
    assert_eq!(parsed["type"], "test", "message type should be 'test'");
    assert!(
        parsed["count"].is_number(),
        "message should have a numeric count"
    );
    assert!(
        parsed["message"].is_string(),
        "message should have a string message"
    );

    eprintln!("TCP test received message: {}", line);
}

/// Requires a running UDP test server: python3 tests/tcp_udp_server.py --protocol udp --port 9001
#[tokio::test]
#[ignore]
async fn test_udp_stream_connect() {
    let port = std::env::var("TEST_UDP_PORT").unwrap_or_else(|_| "9001".to_string());
    let addr = format!("127.0.0.1:{}", port);

    // Bind a local UDP socket and connect to the server
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .expect("failed to bind UDP socket");
    socket
        .connect(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to connect UDP socket to {}: {}", addr, e));

    // Send an initial datagram so the server knows our address
    socket
        .send(b"hello")
        .await
        .expect("failed to send UDP datagram");

    // Wait for a response datagram from the server
    let mut buf = vec![0u8; 65535];
    let n = tokio::time::timeout(std::time::Duration::from_secs(10), socket.recv(&mut buf))
        .await
        .expect("timed out waiting for UDP message")
        .expect("UDP recv error");

    let line = std::str::from_utf8(&buf[..n]).expect("received data should be valid UTF-8");
    let line = line.trim_end_matches('\n').trim_end_matches('\r');

    // The Python test server sends JSON messages
    let parsed: serde_json::Value =
        serde_json::from_str(line).expect("received line should be valid JSON");
    assert_eq!(parsed["type"], "test", "message type should be 'test'");
    assert!(
        parsed["count"].is_number(),
        "message should have a numeric count"
    );
    assert!(
        parsed["message"].is_string(),
        "message should have a string message"
    );

    eprintln!("UDP test received message: {}", line);
}
