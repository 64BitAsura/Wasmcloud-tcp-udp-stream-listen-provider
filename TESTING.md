# TCP/UDP Stream Provider Testing

## Quick Test (Automated)

```bash
./tests/run_integration_test.sh
```

## Manual Test Steps

### Prerequisites

```bash
# Python 3 is required for the test TCP/UDP server
python3 --version
```

### Step 1: Start the Test TCP Server

```bash
python3 tests/tcp_udp_server.py --protocol tcp --port 9000
```

Server listens on `127.0.0.1:9000`, sends JSON messages every 3 seconds.

For UDP testing:

```bash
python3 tests/tcp_udp_server.py --protocol udp --port 9000
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
wash app deploy ./wadm.yaml
```

Or manually:

```bash
wash start provider file://./build/wasmcloud-provider-messaging-tcp-udp-stream.par.gz tcp-udp-stream-provider
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

### Step 6: Verify

Check the wasmCloud host output for:

- `TCP stream connected` or `UDP socket connected`
- `received TCP line: ...` or `received UDP datagram: ...`
- `Message successfully sent to component ...`
- `Received message - Subject: stream.127.0.0.1:9000, Size: ... bytes`

The test server terminal should show client connections.

## Using WADM

You can also deploy the full application declaratively:

```bash
wash up -d
wash app deploy ./wadm.yaml
```

## Testing Edge Cases

### Protocol Switching

Test both TCP and UDP by changing the `protocol` config value:

```bash
wash config put stream-config-udp \
  protocol=udp \
  host=127.0.0.1 \
  port=9000
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
