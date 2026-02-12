# Agent Development Guidelines

This document serves as **living documentation** for the TCP/UDP Stream Listen Provider project. It covers architecture, workflow, and implementation guidelines.

## Architecture Overview

### Provider Role

The TCP/UDP Stream Listen Provider is a **wasmCloud capability provider** that:

1. Connects to a remote TCP or UDP server as a **client**
2. Reads incoming ASCII messages (line-delimited for TCP, per-datagram for UDP)
3. Wraps each message as a `BrokerMessage`
4. Forwards it to linked wasmCloud components via `wasmcloud:messaging/handler.handle-message`

### Message Flow

```
Remote Server → TCP/UDP Stream → Provider → NATS Mesh → wasmCloud Component
```

The provider is **unidirectional** — it only receives messages. Reply-back (sending messages from components to the remote server) is deferred to a future release.

### Key Components

| Module           | Responsibility                                          |
|------------------|---------------------------------------------------------|
| `connection.rs`  | Configuration parsing, merging, protocol selection      |
| `stream.rs`      | Core provider: TCP/UDP readers, link management, wRPC   |
| `main.rs`        | Binary entry point                                      |

### WIT Interface

The provider uses the standard `wasmcloud:messaging@0.2.0` interface:

- **Imports** `wasmcloud:messaging/handler` — to call `handle-message` on components
- **Exports** `wasmcloud:messaging/consumer` — publish/request (deferred, returns error)

## Development Workflow

### Incremental Steps

1. **Scaffold** — Use `wash new provider --template-name messaging-nats` to generate the base structure
2. **Adapt** — Replace NATS-specific code with TCP/UDP stream reading logic
3. **Test** — Write unit tests for config, integration tests for stream
4. **Lint** — Run `cargo fmt` and `cargo clippy` after each step
5. **CI** — GitHub Actions pipeline validates formatting, clippy, build, and tests on every push/PR

### Commit Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[scope]: <description>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`

### Testing Strategy

| Category       | Location                  | Run With           |
|----------------|---------------------------|--------------------|
| Unit tests     | `src/connection.rs`       | `cargo test`       |
| Unit tests     | `src/stream.rs`           | `cargo test`       |
| Integration    | `tests/integration_test.rs`| `cargo test`      |
| Network tests  | `tests/` (ignored)        | `cargo test -- --ignored` |

### Quality Checks

```bash
cargo fmt -- --check   # Formatting
cargo clippy -- -D warnings  # Linting
cargo test             # Unit + integration tests
cargo build --release  # Release build
```

## Implementation Guidelines

### Single Responsibility

Each function should do **one thing**:

- `ConnectionConfig::from_map()` — parse config from HashMap
- `start_stream()` — spawn a background reader for one component
- `dispatch_msg()` — wrap a line as BrokerMessage and forward it

### Error Handling

- Use `anyhow::Result` for fallible operations
- Log errors with `tracing::error!` before returning
- Never panic in production code paths

### Thread Safety

- All shared state uses `Arc<RwLock<T>>`
- Background tasks communicate via channels (oneshot for shutdown)
- `Drop` implementations abort tasks to prevent leaks

### Security

- Validate all external input (config values, received messages)
- No secrets in source code
- Dependency auditing via `cargo audit` and `.cargo/audit.toml`

## Deferred Features

The following are explicitly **out of scope** for the initial release:

- **Reply-back**: Sending messages from components back to the remote TCP/UDP server
- **TLS/SSL**: Encrypted connections
- **Binary messages**: Non-ASCII stream handling
- **Auto-reconnection**: Reconnecting after TCP disconnect
- **Rate limiting**: Per-component message rate limits

These will be addressed in future incremental steps.

## References

- [wasmCloud Documentation](https://wasmcloud.com/docs)
- [wasmCloud Messaging Interface](https://github.com/wasmCloud/wasmCloud/tree/main/wit/wasmcloud-messaging)
- [wash CLI](https://wasmcloud.com/docs/installation)
- [Reference: wasm-cloud-websocket-provider](https://github.com/64BitAsura/wasm-cloud-websocket-provider)
