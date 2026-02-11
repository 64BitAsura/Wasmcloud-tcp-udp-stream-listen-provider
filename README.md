# TCP/UDP Stream Listen Provider for WasmCloud

A WasmCloud capability provider that acts as a unidirectional TCP/UDP stream client. It receives ASCII messages from remote servers and forwards them to WasmCloud components via NATS mesh.

## Features

- ✅ TCP client connection with automatic reconnection
- ✅ UDP client connection
- ✅ ASCII message streaming
- ✅ Message forwarding to NATS mesh
- ✅ Multiple simultaneous connections
- ✅ Configurable via link configuration
- ❌ Reply-back capability (deferred)

## Architecture

```
Remote TCP/UDP Server → Provider Client → Message Handler → NATS Mesh → WasmCloud Component
```

## Configuration

Configure the provider through link configuration with the following structure:

```json
{
  "tcp": [
    {
      "host": "example.com",
      "port": 8080,
      "reconnect_delay_ms": 5000
    }
  ],
  "udp": [
    {
      "host": "example.com",
      "port": 9090
    }
  ]
}
```

### Configuration Options

- **tcp**: Array of TCP connection configurations
  - **host**: Remote server hostname or IP address
  - **port**: Remote server port
  - **reconnect_delay_ms**: Delay before reconnection attempts (default: 5000ms)

- **udp**: Array of UDP connection configurations
  - **host**: Remote server hostname or IP address
  - **port**: Remote server port

## Building

```bash
# Build the provider
cargo build --release

# Run tests
cargo test

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt
```

## Testing

The provider includes unit tests for all core modules:

```bash
cargo test
```

## Integration with WasmCloud

1. **Build the provider**:
   ```bash
   cargo build --release
   ```

2. **Start WasmCloud host**:
   ```bash
   wash up
   ```

3. **Deploy the provider to the lattice** (when provider is packaged):
   ```bash
   wash par create --name tcp-udp-stream-provider
   ```

4. **Link to a component**:
   Configure the link with TCP/UDP connection details as shown in the configuration section.

## Message Format

Messages are forwarded to NATS with the following JSON structure:

```json
{
  "source": "tcp://example.com:8080",
  "content": "ASCII message content",
  "timestamp": 1234567890
}
```

### NATS Topics

Messages are published to topics based on the source:
- Base topic: `wasmcloud.stream.messages`
- Example: `wasmcloud.stream.messages.tcp_example_com_8080`

## Development

See [Agents.md](./Agents.md) for detailed development guidelines, architecture decisions, and implementation phases.

## Dependencies

- `wasmcloud-provider-sdk`: Core WasmCloud provider functionality
- `tokio`: Async runtime
- `async-nats`: NATS client for message forwarding
- `serde`: Configuration serialization
- `tracing`: Logging and diagnostics

## License

Apache-2.0

## Contributing

Contributions are welcome! Please ensure:
1. All tests pass: `cargo test`
2. Code is formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`

## Roadmap

- [x] Basic TCP client
- [x] Basic UDP client
- [x] NATS message forwarding
- [x] Unit tests
- [x] Linting and formatting
- [x] Integration tests with mock servers
- [ ] Deployment tests
- [ ] CI/CD pipeline (in progress)
- [ ] Provider archive (PAR) packaging
- [ ] Connection health monitoring
- [ ] Metrics and observability
- [ ] Reply-back capability (future)

## References

- [WasmCloud Documentation](https://wasmcloud.com/docs)
- [WasmCloud Provider SDK](https://github.com/wasmCloud/wasmCloud)
- [Inspiration: WebSocket Client Provider](https://github.com/64BitAsura/WasmCloud-websocket-client-provider)
