# Tests

This directory contains integration tests and test utilities for the TCP/UDP Stream Listen Provider.

## Running Tests

```bash
# Run all non-ignored tests (unit + integration)
cargo test

# Run ignored tests that require network access
cargo test -- --ignored

# Run the full integration test (requires wash CLI, wasmCloud host)
./tests/run_integration_test.sh
```

## Test Utilities

| File | Description |
|------|-------------|
| `integration_test.rs` | High-level integration tests: build verification, config parsing, TCP/UDP stream tests (ignored by default) |
| `tcp_udp_server.py` | Python test server for TCP/UDP message generation |
| `run_integration_test.sh` | Automated integration test script (format, build, deploy, verify) |

## Test Server Usage

```bash
# Start TCP test server
python3 tests/tcp_udp_server.py --protocol tcp --port 9000

# Start UDP test server
python3 tests/tcp_udp_server.py --protocol udp --port 9000
```

## Unit Tests

Unit tests are located directly in the source modules:
- `src/config.rs` — Configuration parsing, merging, defaults
- `src/provider.rs` — Provider creation, shutdown, publish/request error paths, broker message creation
- `src/stream.rs` — TCP/UDP stream client logic
