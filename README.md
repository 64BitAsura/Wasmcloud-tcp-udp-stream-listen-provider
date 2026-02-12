# TCP/UDP Stream Listen Capability Provider

A wasmCloud capability provider that implements the `wasmcloud:messaging` contract by acting as a **unidirectional TCP/UDP ASCII message stream client**. It connects to a remote TCP or UDP server, reads incoming ASCII messages (line-delimited for TCP, datagram-per-message for UDP), and forwards them to wasmCloud components via wRPC.

## Building

Prerequisites: [Rust toolchain](https://www.rust-lang.org/tools/install), [wash CLI](https://wasmcloud.com/docs/installation)

```bash
# Build the provider (.par.gz archive)
wash build

# Build the test component
wash build -p ./component
```

## Testing

Run the automated integration test:

```bash
./tests/run_integration_test.sh
```

Or deploy as a WADM application:

```bash
wash up -d
wash app deploy ./wadm.yaml
```

See [TESTING.md](./TESTING.md) for detailed manual testing steps.

## Development

For contributing to this project, see [Agents.md](./Agents.md) for the structured implementation process including:
- Analysis of implementation prompts
- Three-solution approach with confidence ratings
- Comprehensive testing checklist (format, clippy, type checks)
- Documentation templates for future reference

## Configuration

| Property        | Description                                                    | Default       |
| :-------------- | :------------------------------------------------------------- | :------------ |
| `protocol`      | Stream protocol: `tcp` or `udp`                                | `tcp`         |
| `host`          | Remote server host                                             | `127.0.0.1`   |
| `port`          | Remote server port                                             | `9000`        |
| `subscriptions` | Comma-separated list of subscription topics (for future use)   | (empty)       |

## Architecture

```
Remote TCP/UDP Server
    │ ASCII messages (line-delimited / datagram)
    ▼
TCP/UDP Stream Provider (Rust + tokio)
    │ wRPC calls via wasmcloud:messaging/handler (over NATS)
    ▼
wasmCloud Component (WebAssembly)
    exports wasmcloud:messaging/handler
```

## Project Structure

```
├── .github/workflows/ci.yml     # CI/CD pipeline
├── src/
│   ├── main.rs                   # Binary entry point
│   ├── config.rs                 # Configuration structs
│   ├── provider.rs               # Provider trait impl + wRPC dispatch
│   └── stream.rs                 # TCP/UDP stream client logic
├── component/
│   ├── src/lib.rs                # Test component implementation
│   ├── wit/                      # Component WIT definitions
│   ├── Cargo.toml
│   └── wasmcloud.toml
├── wit/
│   ├── world.wit                 # Provider WIT world definition
│   └── deps/                     # WIT dependencies
├── tests/
│   ├── integration_test.rs       # Integration tests
│   ├── tcp_udp_server.py         # Python test server
│   └── run_integration_test.sh   # Automated test script
├── Agents.md                     # Living documentation
├── Cargo.toml
├── wadm.yaml                     # WADM deployment manifest
├── wasmcloud.toml                # wasmCloud provider metadata
├── TESTING.md                    # Testing guide
└── README.md
```

## Current Limitations

- **Unidirectional only**: The provider receives messages; reply-back is deferred
- **ASCII only**: Binary streams are not parsed (UDP datagrams must be valid UTF-8)
- **No reconnection**: If the TCP connection drops, the stream task exits
- **No TLS**: Plain TCP/UDP only in this release

## Future Enhancements

- [ ] Reply-back feature (send messages from component to remote server)
- [ ] Automatic reconnection with exponential backoff
- [ ] TLS support
- [ ] Binary message support
- [ ] Connection health checks
- [ ] Metrics and observability

## License

Apache-2.0 — See [LICENSE](LICENSE)