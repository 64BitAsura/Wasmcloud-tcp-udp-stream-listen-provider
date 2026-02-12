# Technical Documentation

## Architecture Overview

The TCP/UDP Stream Listen Provider implements the `wasmcloud:messaging` contract using raw TCP/UDP sockets as the transport layer. It is designed as a **unidirectional receiver** — it reads ASCII messages from a remote server and forwards them to wasmCloud components.

## Core Components

### 1. Connection Configuration (`connection.rs`)

```rust
pub struct ConnectionConfig {
    pub protocol: StreamProtocol,    // tcp or udp
    pub host: String,                // Remote server host
    pub port: u16,                   // Remote server port
    pub subscriptions: Vec<String>,  // Topic list (future use)
}
```

- Parsed from `HashMap<String, String>` (wasmCloud link config)
- Supports config merging (default + link-specific)
- Protocol selection: TCP (line-delimited) or UDP (datagram)

### 2. Stream Provider (`stream.rs`)

```rust
pub struct TcpUdpStreamProvider {
    components: Arc<RwLock<HashMap<String, StreamBundle>>>,
    default_config: ConnectionConfig,
}
```

- One `StreamBundle` per linked component
- Each bundle contains a background tokio task + shutdown signal
- Tasks are aborted on unlink or provider shutdown

### 3. Message Flow

#### TCP Mode
1. `TcpStream::connect(addr)` connects to remote server
2. `BufReader::lines()` reads newline-delimited ASCII
3. Each line → `BrokerMessage { subject: "stream.message", body: line_bytes }`
4. Dispatched to component via `wasmcloud:messaging/handler.handle-message`

#### UDP Mode
1. `UdpSocket::bind("0.0.0.0:0")` + `connect(addr)` for connected recv
2. `socket.recv(&mut buf)` reads datagrams
3. Each valid UTF-8 datagram → `BrokerMessage`
4. Non-UTF-8 datagrams are skipped with a debug log

### 4. Link Lifecycle

- **receive_link_config_as_target**: Spawns TCP/UDP reader for the component
- **delete_link_as_target**: Drops `StreamBundle`, aborting the reader task
- **shutdown**: Clears all components, aborting all tasks

## Thread Safety

All shared state uses `Arc<RwLock<T>>`:
- Multiple readers can access the component map concurrently
- Writers get exclusive access during link/unlink

Background tasks use `tokio::sync::oneshot` for shutdown signaling.

## Error Handling

- `anyhow::Result` throughout
- Connection failures are logged and returned to the host
- Stream read errors terminate the task (reconnection deferred)
- Config parsing failures use defaults for missing values

## WIT Interface

```wit
package wasmcloud:provider-messaging-tcp-udp-stream;

world provider-messaging-tcp-udp-stream {
    import wasmcloud:messaging/handler@0.2.0;
    export wasmcloud:messaging/consumer@0.2.0;
}
```

The provider **imports** `handler` to call components, and **exports** `consumer` (publish/request return errors since the provider is receive-only).

## Performance Considerations

- Each component gets its own TCP connection or UDP socket
- Background tasks run on the tokio runtime
- Line parsing for TCP uses `BufReader` for efficient buffered I/O
- UDP uses a 64KB buffer per socket

## Future Enhancements

1. Reply-back (publish/request to remote server)
2. Auto-reconnection with exponential backoff
3. TLS support
4. Binary message handling
5. Connection health monitoring
6. Metrics integration
