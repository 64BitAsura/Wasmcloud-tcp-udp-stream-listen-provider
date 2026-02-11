# wasmCloud TCP/UDP Stream Listener Provider

A wasmCloud capability provider that listens for TCP and UDP ASCII message streams and forwards them to components via the `wasmcloud:messaging` interface.

## Features

- **TCP Listener**: Accepts TCP connections and streams ASCII messages
- **UDP Listener**: Receives UDP datagrams with ASCII messages
- **Configurable Ports**: Set custom TCP and UDP ports via link configuration
- **Message Forwarding**: Forwards messages to linked components using wasmcloud:messaging interface
- **Multi-line Support**: Handles multiple lines in a single message

## Project Structure

```
.
├── provider/          # TCP/UDP stream listener provider implementation
├── component/         # Example component that receives messages
├── test-server/       # Test message generator (TCP/UDP sender)
├── wit/              # WIT interface definitions
└── wasmcloud.yaml    # wasmCloud application manifest
```

## Prerequisites

- Rust 1.70+ with `wasm32-wasi` target
- wasmCloud CLI (`wash`) - for building and running
- Docker (optional, for containerized testing)

## Building

### 1. Build the Provider

```bash
cd provider
cargo build --release
```

### 2. Build the Component

```bash
cd component
cargo build --release --target wasm32-wasi
```

### 3. Build the Test Server

```bash
cd test-server
cargo build --release
```

## Configuration

The provider accepts the following link configuration parameters:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `enable_tcp` | boolean | `true` | Enable TCP listener |
| `enable_udp` | boolean | `true` | Enable UDP listener |
| `tcp_port` | integer | `8080` | TCP listening port |
| `udp_port` | integer | `8081` | UDP listening port |

## Usage

### Running with wasmCloud

1. Start wasmCloud host:
```bash
wash up
```

2. Deploy the application:
```bash
wash app deploy wasmcloud.yaml
```

3. Send test messages using the test server:

**TCP messages:**
```bash
cd test-server
cargo run -- tcp --host 127.0.0.1 --port 8080 --count 10 --interval 1000
```

**UDP messages:**
```bash
cargo run -- udp --host 127.0.0.1 --port 8081 --count 10 --interval 1000
```

**Both TCP and UDP:**
```bash
cargo run -- both --host 127.0.0.1 --tcp-port 8080 --udp-port 8081 --count 10
```

### Test Server Options

The test server supports the following commands:

- `tcp`: Send messages via TCP
- `udp`: Send messages via UDP
- `both`: Send messages via both protocols simultaneously

Common options:
- `-h, --host`: Target host (default: 127.0.0.1)
- `-p, --port`: Target port
- `-c, --count`: Number of messages (0 for infinite)
- `-i, --interval`: Interval between messages in ms
- `-m, --message`: Message prefix

### Testing with netcat

You can also test with standard tools like netcat:

**TCP:**
```bash
echo "Hello from TCP" | nc 127.0.0.1 8080
```

**UDP:**
```bash
echo "Hello from UDP" | nc -u 127.0.0.1 8081
```

## Development

### Running Tests

```bash
cargo test --workspace
```

### Adding Custom Logic

To customize message processing in the component, edit `component/src/lib.rs`:

```rust
impl Guest for StreamReceiver {
    fn handle_message(msg: BrokerMessage) -> Result<(), String> {
        let body = String::from_utf8(msg.body)
            .map_err(|e| format!("Failed to parse: {}", e))?;
        
        // Add your custom logic here
        process_message(&body);
        
        Ok(())
    }
}
```

## Architecture

```
┌─────────────────┐
│  TCP/UDP Client │
│   (test-server) │
└────────┬────────┘
         │
         │ ASCII Messages
         ▼
┌─────────────────────┐
│  TCP/UDP Provider   │
│  (Rust Provider)    │
│  - TCP Listener     │
│  - UDP Listener     │
└────────┬────────────┘
         │
         │ wasmcloud:messaging
         ▼
┌─────────────────────┐
│  Stream Receiver    │
│  (WASM Component)   │
│  - Message Handler  │
└─────────────────────┘
```

## Message Format

Messages are expected to be ASCII text, with each line treated as a separate message. The provider:
1. Receives data from TCP/UDP
2. Converts to UTF-8 string
3. Splits by newlines
4. Sends each non-empty line to the component

Example:
```
Message 1
Message 2
Message 3
```

Each line is delivered as a separate `BrokerMessage` to the component.

## Troubleshooting

### Provider not receiving messages

1. Check that the ports are not in use:
   ```bash
   lsof -i :8080
   lsof -i :8081
   ```

2. Verify the provider is linked to the component:
   ```bash
   wash get links
   ```

3. Check provider logs:
   ```bash
   wash logs
   ```

### Component not processing messages

1. Check component logs in wasmCloud host output
2. Verify the component is running:
   ```bash
   wash get inventory
   ```

## License

See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.