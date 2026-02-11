#!/bin/bash
set -e

echo "Building Test Server..."

cd test-server
cargo build --release
cd ..

echo "Test server build complete!"
echo "Binary: test-server/target/release/test-server"
