# TCP/UDP Stream Provider Testing

## Quick Test (Automated)

```bash
./tests/run_integration_test.sh
```

This script performs:
1. Format and lint checks (`cargo fmt`, `cargo clippy`)
2. Provider build (`wash build`)
3. Component build (`wash build -p ./component`)
4. Unit tests (`cargo test`)
5. Starts a TCP test server (Python)
6. Starts a wasmCloud host with logs captured to file
7. Deploys the provider and component to the host
8. Creates a config and link between the component and provider
9. Monitors wasmCloud logs for 30 seconds
10. **Checks logs for provider-level messages** (`TCP stream connected`, `Message successfully sent to component`)
11. **Checks logs for component-level messages** (`Received message`)
12. Reports PASS/FAIL based on log analysis

## Manual Test Steps

### Prerequisites

```bash
# wash CLI
wash --version

# Python 3 is required for the test TCP/UDP server
python3 --version

# Rust toolchain with wasm targets
rustup target add wasm32-unknown-unknown
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
wash build -p ./component
```

The provider archive will be in `build/` (`.par.gz`), the component in `component/build/` (`.wasm`).

### Step 3: Start wasmCloud Host

```bash
wash up
```

Wait until `wash get hosts` shows a host ID.

### Step 4: Deploy Provider and Component

```bash
wash start provider file://./build/tcp-udp-stream-provider.par.gz tcp-udp-stream-provider
wash start component file://./component/build/tcp_udp_stream_test_component.wasm test-component
```

Verify both are running:

```bash
wash get inventory
```

### Step 5: Create Config and Link

```bash
# Create named config
wash config put stream-config \
  protocol=tcp \
  host=127.0.0.1 \
  port=9000

# Link component to provider
wash link put test-component tcp-udp-stream-provider \
  wasmcloud messaging \
  --interface handler \
  --target-config stream-config
```

### Step 6: Verify via Logs

Check the wasmCloud host output for:

- `TCP stream connected` or `UDP socket connected` — provider connected to server
- `received TCP line` or `received UDP datagram` — provider reading data
- `Message successfully sent to component` — provider forwarded message via wRPC
- `Received message - Subject: stream.127.0.0.1:9000, Size: ... bytes` — component processed the message

The test server terminal should show client connections.

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

## Cleanup

```bash
wash down
```

## Troubleshooting

| Problem | Check |
|---------|-------|
| Provider not connecting | Is the TCP/UDP server running? Check `host` and `port` in config |
| Component not receiving messages | Run `wash link query` and `wash get inventory` |
| NATS connection issues | Run `wash get hosts`, try `wash down && wash up` |

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
