# Project Structure

This document describes the project structure, which follows WasmCloud wash CLI scaffolding conventions.

## Directory Structure

```
Wasmcloud-tcp-udp-stream-listen-provider/
├── .github/
│   └── workflows/
│       └── ci.yml                    # CI/CD pipeline
├── src/                              # Provider implementation
│   ├── lib.rs                        # Main provider library
│   ├── main.rs                       # Binary entry point
│   ├── connection.rs                 # TCP/UDP connection management
│   ├── handler.rs                    # Message handling
│   └── nats.rs                       # NATS publisher
├── tests/                            # Integration tests
│   └── integration_tests.rs          # TCP/UDP integration tests
├── wit/                              # WIT interface definitions
│   └── world.wit                     # TCP/UDP stream interface
├── wasmcloud-example/                # Example deployment
│   ├── consumer-component/           # Example consumer component
│   │   ├── src/
│   │   │   └── lib.rs
│   │   ├── wit/
│   │   │   └── world.wit
│   │   ├── Cargo.toml
│   │   └── wasmcloud.toml
│   ├── wadm.yaml                     # Application manifest
│   ├── Makefile                      # Build commands
│   ├── test-local.sh                 # Quick test script
│   └── README.md
├── examples/                         # Usage examples
│   └── README.md
├── Cargo.toml                        # Root Cargo manifest
├── wasmcloud.toml                    # Provider metadata
├── README.md                         # Main documentation
├── QUICKSTART.md                     # Quick start guide
├── CONFIG.md                         # Configuration reference
├── CONTRIBUTING.md                   # Contribution guide
├── Agents.md                         # Living documentation
└── LICENSE

```

## Comparison with Reference Structure

This structure closely matches the [wasm-cloud-websocket-provider](https://github.com/64BitAsura/wasm-cloud-websocket-provider):

| Component | This Project | WebSocket Provider | Status |
|-----------|-------------|-------------------|---------|
| Provider code in `src/` | ✅ | ✅ | Matches |
| Tests in `tests/` | ✅ | ✅ | Matches |
| WIT interfaces | ✅ | ✅ | Matches |
| Example component | ✅ | ✅ | Matches |
| wadm.yaml | ✅ | ✅ | Matches |
| Root Cargo.toml | ✅ | ✅ | Matches |
| CI/CD pipeline | ✅ | ✅ | Matches |
| Documentation | ✅ | ✅ | Matches |

## Key Files

### Provider Implementation

- **src/lib.rs**: Main provider implementation with `Provider` trait
- **src/main.rs**: Binary entry point for standalone execution
- **src/connection.rs**: TCP/UDP client connection management
- **src/handler.rs**: Message processing and forwarding
- **src/nats.rs**: NATS publisher for message distribution

### WIT Interfaces

- **wit/world.wit**: Defines the TCP/UDP stream message interface
  - `stream-message` record type
  - `handle-message` function
  - `tcp-udp-stream-world` world

### Example Component

- **wasmcloud-example/consumer-component/**: Full WasmCloud component
  - Generated with `wash new component`
  - Demonstrates message consumption
  - Ready for deployment

### Configuration

- **wasmcloud.toml**: Provider metadata for wash CLI
- **Cargo.toml**: Rust package configuration with binary target
- **wasmcloud-example/wadm.yaml**: Application deployment manifest

### Testing

- **tests/integration_tests.rs**: Integration tests with mock TCP/UDP servers
- **Unit tests**: Embedded in source files (10 tests)
- **CI/CD**: Automated testing in GitHub Actions

## Build & Deploy

### Build Provider

```bash
cargo build --release
```

Binary output: `target/release/tcp-udp-stream-provider`

### Build Example Component

```bash
cd wasmcloud-example/consumer-component
wash build
```

### Deploy Complete Application

```bash
cd wasmcloud-example
./test-local.sh
```

Or manually:

```bash
wash app deploy wasmcloud-example/wadm.yaml
```

## Differences from Original Structure

The project was restructured from a workspace layout to a flat structure:

### Old Structure (❌ Removed)
```
├── provider/
│   ├── Cargo.toml
│   ├── src/
│   └── tests/
├── Cargo.toml (workspace)
└── wadm/
    └── example-app.yaml
```

### New Structure (✅ Current)
```
├── src/              # Direct at root
├── tests/            # Direct at root
├── Cargo.toml        # Single package
└── wasmcloud-example/
    └── wadm.yaml
```

### Benefits of New Structure

1. **Simpler**: No workspace complexity
2. **Standard**: Follows wash CLI conventions
3. **Clear**: Provider and component clearly separated
4. **Professional**: Matches established WasmCloud projects
5. **Deployable**: Ready for PAR creation with `wash par create`

## Next Steps

To extend this project:

1. **Add more examples**: Create additional consumer components
2. **Enhance provider**: Add TLS support, authentication, etc.
3. **Create PAR**: Package with `wash par create`
4. **Publish**: Share on wasmCloud registry

## References

- [WasmCloud Documentation](https://wasmcloud.com/docs)
- [Wash CLI Guide](https://wasmcloud.com/docs/cli)
- [WebSocket Provider Reference](https://github.com/64BitAsura/wasm-cloud-websocket-provider)
