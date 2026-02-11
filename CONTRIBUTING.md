# Contributing to TCP/UDP Stream Provider

Thank you for your interest in contributing to the TCP/UDP Stream Provider for WasmCloud!

## Getting Started

1. **Fork and Clone**:
   ```bash
   git clone https://github.com/64BitAsura/Wasmcloud-tcp-udp-stream-listen-provider
   cd Wasmcloud-tcp-udp-stream-listen-provider
   ```

2. **Install Dependencies**:
   - Rust (latest stable): https://rustup.rs/
   - wash CLI: 
     ```bash
     curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | sudo bash
     sudo apt install wash
     ```

3. **Build the Project**:
   ```bash
   cargo build
   ```

4. **Run Tests**:
   ```bash
   cargo test
   ```

## Development Workflow

### Making Changes

1. **Create a Branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes** following the code style guidelines below.

3. **Write Tests**: Add unit and/or integration tests for your changes.

4. **Run Quality Checks**:
   ```bash
   # Format code
   cargo fmt
   
   # Run linter
   cargo clippy --all-targets --all-features -- -D warnings
   
   # Run tests
   cargo test
   ```

5. **Commit Your Changes**:
   ```bash
   git add .
   git commit -m "Brief description of your changes"
   ```

6. **Push and Create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request on GitHub.

## Code Style Guidelines

- Follow Rust standard formatting (enforced by `cargo fmt`)
- No clippy warnings (run `cargo clippy`)
- Add documentation comments for public APIs
- Write descriptive commit messages
- Keep functions focused and modular

## Testing Guidelines

### Unit Tests
- Place unit tests in the same file as the code they test
- Use the `#[cfg(test)]` module pattern
- Test both success and error cases

### Integration Tests
- Place integration tests in `provider/tests/`
- Test interactions between modules
- Use mock servers for TCP/UDP testing

### Running Specific Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests
```

## Project Structure

```
.
├── Agents.md              # Living documentation
├── README.md              # Main documentation
├── CONTRIBUTING.md        # This file
├── Cargo.toml             # Workspace manifest
├── provider/              # Provider implementation
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs         # Main provider logic
│   │   ├── connection.rs  # TCP/UDP connection management
│   │   ├── handler.rs     # Message handling
│   │   └── nats.rs        # NATS integration
│   └── tests/             # Integration tests
├── examples/              # Usage examples
├── wadm/                  # WasmCloud application manifests
└── .github/
    └── workflows/         # CI/CD pipelines
```

## Areas for Contribution

### High Priority
- [ ] Connection health monitoring and metrics
- [ ] Reconnection logic improvements
- [ ] Provider archive (PAR) packaging
- [ ] Comprehensive deployment testing
- [ ] Performance benchmarks

### Future Features
- [ ] Bidirectional communication (reply-back)
- [ ] Message transformation/filtering
- [ ] TLS/SSL support
- [ ] Authentication mechanisms
- [ ] Protocol buffer support

### Documentation
- [ ] API documentation improvements
- [ ] More deployment examples
- [ ] Troubleshooting guide
- [ ] Architecture diagrams

## Reporting Issues

When reporting issues, please include:
- Rust version: `rustc --version`
- wash version: `wash --version`
- Operating system and version
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs

## Code Review Process

1. All changes require review before merging
2. CI must pass (tests, linting, formatting)
3. Maintainers will review and provide feedback
4. Address feedback and update PR
5. Once approved, maintainer will merge

## Living Documentation

We maintain living documentation in `Agents.md`. When making architectural changes:
1. Update the relevant section in Agents.md
2. Keep the implementation phase checklist current
3. Document design decisions and tradeoffs

## Questions?

- Open an issue for bugs or feature requests
- Discussions: Use GitHub Discussions for questions
- Check existing issues before creating new ones

## License

By contributing, you agree that your contributions will be licensed under the Apache-2.0 License.

---

Thank you for contributing! 🎉
