# Contributing to TCP/UDP Stream Listen Provider

Thank you for your interest in contributing!

## Development Setup

### Prerequisites

- Rust 1.70 or later
- [wash CLI](https://wasmcloud.com/docs/installation)
- Git

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/64BitAsura/Wasmcloud-tcp-udp-stream-listen-provider.git
cd Wasmcloud-tcp-udp-stream-listen-provider
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## Project Structure

```
├── src/
│   ├── main.rs          # Binary entry point
│   ├── connection.rs    # Connection configuration
│   └── stream.rs        # Core TCP/UDP provider logic
├── tests/
│   └── integration_test.rs  # Integration tests
├── examples/
│   └── basic_usage.rs   # Example usage
├── wit/                 # WebAssembly Interface Types
│   ├── provider.wit
│   └── deps/
└── .github/
    └── workflows/
        └── ci.yml       # CI/CD configuration
```

## Making Changes

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Fix all clippy warnings (`cargo clippy -- -D warnings`)
- Add tests for new features
- Update documentation as needed

### Testing

```bash
# Run all non-ignored tests
cargo test

# Run with output
cargo test -- --nocapture

# Run ignored network tests (requires servers)
cargo test -- --ignored
```

### Submitting Changes

1. Create a feature branch: `git checkout -b feat/your-feature`
2. Make changes and commit following conventional commits
3. Push and create a Pull Request
4. CI must pass before merging

## Commit Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(stream): add TCP reconnection logic
fix(config): handle missing port gracefully
docs(readme): update configuration table
test(integration): add UDP stream test
```

## Reporting Issues

Please include:
- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior

## License

Apache-2.0 — See [LICENSE](LICENSE)
