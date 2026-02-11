#!/bin/bash
set -e

echo "Building TCP/UDP Stream Provider..."

# Build provider
echo "Building provider..."
cd provider
cargo build --release
cd ..

echo "Provider build complete!"
