.PHONY: all build build-provider build-component build-test-server test clean run-test-tcp run-test-udp run-test-both help

# Default target
all: build

# Build all components
build: build-provider build-component build-test-server

# Build the provider
build-provider:
	@echo "Building TCP/UDP stream provider..."
	cd provider && cargo build --release

# Build the component
build-component:
	@echo "Building stream receiver component..."
	@rustup target add wasm32-wasip1 2>/dev/null || true
	cd component && cargo build --release --target wasm32-wasip1

# Build the test server
build-test-server:
	@echo "Building test server..."
	cd test-server && cargo build --release

# Run tests
test:
	@echo "Running tests..."
	cargo test --workspace

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Run TCP test messages (10 messages)
run-test-tcp:
	@echo "Sending TCP test messages..."
	cd test-server && cargo run --release -- tcp --count 10 --interval 500

# Run UDP test messages (10 messages)
run-test-udp:
	@echo "Sending UDP test messages..."
	cd test-server && cargo run --release -- udp --count 10 --interval 500

# Run both TCP and UDP test messages
run-test-both:
	@echo "Sending TCP and UDP test messages..."
	cd test-server && cargo run --release -- both --count 10 --interval 500

# Show help
help:
	@echo "Available targets:"
	@echo "  make build              - Build all components"
	@echo "  make build-provider     - Build only the provider"
	@echo "  make build-component    - Build only the component"
	@echo "  make build-test-server  - Build only the test server"
	@echo "  make test               - Run tests"
	@echo "  make clean              - Clean build artifacts"
	@echo "  make run-test-tcp       - Send TCP test messages"
	@echo "  make run-test-udp       - Send UDP test messages"
	@echo "  make run-test-both      - Send both TCP and UDP test messages"
	@echo "  make help               - Show this help message"
