# Tests

This directory contains integration tests for the TCP/UDP Stream Listen Provider.

## Running Tests

```bash
# Run all non-ignored tests (unit + integration)
cargo test

# Run ignored tests that require network access
cargo test -- --ignored
```

## Test Categories

| File | Description |
|------|-------------|
| `integration_test.rs` | High-level integration tests: build verification, config parsing, TCP/UDP stream tests (ignored by default) |

Unit tests are located directly in the source modules:
- `src/connection.rs` — Configuration parsing, merging, defaults
- `src/stream.rs` — Provider creation, shutdown, publish/request error paths
