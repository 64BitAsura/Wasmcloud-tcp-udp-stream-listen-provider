#!/bin/bash
set -e

echo "Building Stream Receiver Component..."

# Add wasm32-wasip1 target if not present
rustup target add wasm32-wasip1 2>/dev/null || true

# Build component
echo "Building component..."
cd component
cargo build --release --target wasm32-wasip1
cd ..

echo "Component build complete!"
echo "WASM file: component/target/wasm32-wasip1/release/stream_receiver.wasm"
