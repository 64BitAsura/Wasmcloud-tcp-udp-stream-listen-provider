# TCP/UDP Stream Provider Testing

## Quick Test (Automated)

```bash
./tests/run_integration_test.sh
```

This script uses **wash v2** CLI and performs:
1. Format and lint checks (`cargo fmt`, `cargo clippy`)
2. Provider build (`wash build`)
3. Component build (`wash -C ./component build`)
4. Unit tests (`cargo test`)
5. TCP stream connectivity test (against Python test server)
6. UDP stream connectivity test (against Python test server)

## Manual Test Steps

### Prerequisites

```bash
# wash v2 CLI
wash --version

# Python 3 is required for the test TCP/UDP server
python3 --version

# Rust toolchain with wasm32-wasip2 target (for component builds)
rustup target add wasm32-wasip2
```

### Step 1: Start the Test TCP Server

```bash
python3 tests/tcp_udp_server.py --protocol tcp --port 9000
```

Server listens on `127.0.0.1:9000`, sends JSON messages every 3 seconds.

For UDP testing:

```bash
python3 tests/tcp_udp_server.py --protocol udp --port 9001
```

### Step 2: Build Provider and Component

```bash
wash build
wash -C ./component build
```

The provider binary will be in `target/release/`, the component wasm in `component/target/wasm32-wasip2/release/`.

### Step 3: Run Integration Tests

With the test server running:

```bash
# TCP integration test
TEST_TCP_PORT=9000 cargo test test_tcp_stream_connect -- --ignored

# UDP integration test
TEST_UDP_PORT=9001 cargo test test_udp_stream_connect -- --ignored
```

### Step 4: Run with wash dev (Component Development)

For developing the test component with wash v2:

```bash
wash -C ./component dev
```

## Testing Edge Cases

### Protocol Switching

Test both TCP and UDP by starting servers on different ports:

```bash
python3 tests/tcp_udp_server.py --protocol tcp --port 9000
python3 tests/tcp_udp_server.py --protocol udp --port 9001
```

### Connection Loss

1. Stop the TCP server (Ctrl+C)
2. Observe the stream task exits in provider logs
3. Restart the server — a new link is needed to reconnect (auto-reconnection is a future feature)

## Troubleshooting

| Problem | Check |
|---------|-------|
| Provider not connecting | Is the TCP/UDP server running? Check `host` and `port` in config |
| `wash build` fails with "build.command is required" | Ensure `.wash/config.yaml` exists with build configuration |
| Component build fails with missing target | Run `rustup target add wasm32-wasip2` |

## Architecture

```
TCP/UDP Server (127.0.0.1:9000)
    │ ASCII messages
    ▼
TCP/UDP Stream Provider (Rust + tokio)
    │ wRPC calls (via NATS)
    ▼
wasmCloud Component (WebAssembly)
```
