# Tests

This directory contains integration tests and test utilities for the TCP/UDP Stream Listen Provider.

## Running Tests

```bash
# Run all non-ignored tests (unit + integration)
cargo test

# Run ignored tests that require network access (start test servers first)
cargo test -- --ignored

# Run the full integration test (requires wash CLI, deploys to wasmCloud host)
./tests/run_integration_test.sh
```

## Test Utilities

| File | Description |
|------|-------------|
| `integration_test.rs` | High-level integration tests: build verification, config parsing, TCP/UDP stream connectivity tests (ignored by default) |
| `tcp_udp_server.py` | Python test server for TCP/UDP message generation |
| `run_integration_test.sh` | Automated integration test script: builds, deploys to wasmCloud host, and verifies message flow by checking host logs at provider and component level |

## Integration Test Script

The `run_integration_test.sh` script deploys the provider and component to a real wasmCloud host,
then checks the host logs for evidence of message flow:

- **Provider-level**: `TCP stream connected`, `Message successfully sent to component`
- **Component-level**: `Received message - Subject: stream.127.0.0.1:9000`

This follows the same approach as the [WasmCloud-websocket-client-provider](https://github.com/64BitAsura/WasmCloud-websocket-client-provider) integration tests.

## Test Server Usage

```bash
# Start TCP test server
python3 tests/tcp_udp_server.py --protocol tcp --port 9000

# Start UDP test server
python3 tests/tcp_udp_server.py --protocol udp --port 9001
```

## Integration Tests (Ignored by Default)

The `test_tcp_stream_connect` and `test_udp_stream_connect` tests require running test servers.
Use environment variables to configure ports:

```bash
# Run with test servers on custom ports
TEST_TCP_PORT=9000 cargo test test_tcp_stream_connect -- --ignored
TEST_UDP_PORT=9001 cargo test test_udp_stream_connect -- --ignored
```

## Unit Tests

Unit tests are located directly in the source modules:
- `src/config.rs` — Configuration parsing, merging, defaults
- `src/provider.rs` — Provider creation, shutdown, publish/request error paths, broker message creation
- `src/stream.rs` — TCP/UDP stream client logic
