# Wasmcloud TCP/UDP Stream Listen Provider

A wasmCloud capability provider that implements the `wasmcloud:messaging` contract by acting as a **unidirectional TCP/UDP ASCII message stream client**. It connects to a remote TCP or UDP server, reads incoming ASCII messages (line-delimited for TCP, datagram-per-message for UDP), and forwards them to wasmCloud components via the NATS mesh.

## Features

- **TCP Stream Client**: Connect to a remote TCP server and read newline-delimited ASCII messages
- **UDP Datagram Client**: Bind a local UDP socket, connect to a remote server, and receive ASCII datagrams
- **wasmCloud Integration**: Implements `wasmcloud:messaging/handler` to forward messages to linked components
- **Session Management**: Per-component stream connections with automatic cleanup on unlink
- **Configurable**: Protocol, host, port, and subscriptions configurable via link config

## Configuration

| Property        | Description                                                    | Default       |
| :-------------- | :------------------------------------------------------------- | :------------ |
| `protocol`      | Stream protocol: `tcp` or `udp`                                | `tcp`         |
| `host`          | Remote server host                                             | `127.0.0.1`   |
| `port`          | Remote server port                                             | `9000`        |
| `subscriptions` | Comma-separated list of subscription topics (for future use)   | (empty)       |

## Quick Start

### Prerequisites

- Rust 1.70+
- [wash CLI](https://wasmcloud.com/docs/installation)

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

### Deploy with wasmCloud

```bash
wash app deploy local.wadm.yaml
```

See [QUICKSTART.md](QUICKSTART.md) for a full walkthrough.

## Architecture

```
Remote TCP/UDP Server
        │
        ▼ (ASCII messages)
┌─────────────────────────┐
│  TCP/UDP Stream Provider│
│  (this crate)           │
│                         │
│  Reads lines/datagrams  │
│  Wraps as BrokerMessage │
└────────┬────────────────┘
         │ wasmcloud:messaging/handler.handle-message
         ▼
┌─────────────────────────┐
│  wasmCloud Component    │
│  (linked via NATS mesh) │
└─────────────────────────┘
```

## Project Structure

```
├── .cargo/audit.toml        # Cargo audit configuration
├── .github/workflows/
│   └── ci.yml               # CI/CD pipeline
├── examples/
│   └── basic_usage.rs       # Example usage
├── src/
│   ├── connection.rs        # Connection configuration
│   ├── main.rs              # Binary entry point
│   └── stream.rs            # Core provider logic (TCP/UDP)
├── tests/
│   └── integration_test.rs  # Integration tests
├── wit/                     # WIT interface definitions
│   ├── provider.wit
│   └── deps/
├── Agents.md                # Living documentation
├── Cargo.toml
├── local.wadm.yaml          # Local deployment manifest
├── wasmcloud.toml            # wasmCloud provider metadata
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