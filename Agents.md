# Agents.md - Living Documentation

## Project: WasmCloud TCP/UDP Stream Listen Provider

### Overview
A WasmCloud capability provider that acts as a unidirectional TCP/UDP client, receiving ASCII messages from remote servers and forwarding them to WasmCloud components via NATS mesh.

### Architecture

#### Components
1. **Provider Core** (`src/lib.rs`)
   - Implements `CapabilityProvider` trait
   - Manages lifecycle (start, stop)
   - Handles configuration and initialization

2. **Connection Manager** (`src/connection.rs`)
   - Manages TCP/UDP client connections
   - Handles reconnection logic
   - Connection pooling for multiple endpoints

3. **Message Handler** (`src/handler.rs`)
   - Receives messages from TCP/UDP streams
   - Parses ASCII messages
   - Forwards to NATS mesh

4. **NATS Integration** (`src/nats.rs`)
   - Publishes messages to WasmCloud components
   - Handles NATS connection and topics

#### Data Flow
```
Remote TCP/UDP Server → Provider Client → Message Handler → NATS Mesh → WasmCloud Component
```

### Implementation Guidelines

#### Phase 1: Project Setup ✓
- [x] Create Cargo workspace structure
- [x] Set up basic dependencies
- [x] Create Agents.md documentation
- [x] Add CI/CD configuration

#### Phase 2: Core Provider Implementation ✓
- [x] Implement basic provider structure
- [x] Add configuration parsing
- [x] Implement provider lifecycle methods
- [x] Add unit tests for provider core

#### Phase 3: Connection Management ✓
- [x] Implement TCP client connection
- [x] Implement UDP client connection
- [x] Add connection health checks
- [x] Add reconnection logic (framework in place)
- [x] Add unit tests for connection handlers

#### Phase 4: Message Processing ✓
- [x] Implement message receiver loop
- [x] Add ASCII message parsing
- [x] Handle message buffering
- [x] Add unit tests for message handling

#### Phase 5: NATS Integration ✓
- [x] Implement NATS publisher
- [x] Add topic routing
- [x] Handle NATS errors
- [x] Add integration tests (basic)

#### Phase 6: Testing & Validation (In Progress)
- [x] Unit tests for all modules
- [ ] Integration tests with mock servers
- [ ] Deployment test with WasmCloud
- [ ] Load testing

#### Phase 7: CI/CD & Documentation ✓
- [x] GitHub Actions workflow
- [x] Linting and formatting checks
- [x] README with examples
- [x] API documentation (inline)

### Configuration Format

```toml
# Provider configuration
[provider]
name = "tcp-udp-stream-listener"
version = "0.1.0"

# TCP connections
[[tcp]]
host = "example.com"
port = 8080
reconnect_delay_ms = 5000

# UDP connections
[[udp]]
host = "example.com"
port = 9090
```

### Dependencies
- `wasmcloud-provider-sdk`: Core WasmCloud provider functionality
- `tokio`: Async runtime
- `async-nats`: NATS client
- `serde`: Configuration serialization
- `tracing`: Logging and diagnostics

### Deferred Features
- **Reply-back capability**: Bidirectional communication (future enhancement)
- **Message transformation**: Complex message parsing (future enhancement)
- **Multiple protocol support**: Beyond TCP/UDP (future enhancement)

### Testing Strategy
1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test with mock TCP/UDP servers
3. **Deployment Tests**: Deploy in actual WasmCloud environment
4. **CI Tests**: Automated testing on every commit

### Workflow

#### Development Cycle
1. Write code for specific module
2. Run `cargo fmt` and `cargo clippy`
3. Write unit tests
4. Run tests: `cargo test`
5. Commit changes
6. CI runs full test suite

#### Deployment Workflow
1. Build provider: `cargo build --release`
2. Create provider archive: `wash par create`
3. Push to registry
4. Deploy to WasmCloud lattice: `wash up`
5. Link to component

### References
- [WasmCloud Provider SDK](https://github.com/wasmCloud/wasmCloud)
- [WebSocket Client Provider](https://github.com/64BitAsura/WasmCloud-websocket-client-provider) - Structure inspiration
- [NATS Documentation](https://docs.nats.io/)

---
*Last Updated: 2026-02-11*
