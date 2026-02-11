#!/bin/bash
set -e

echo "🚀 TCP/UDP Stream Provider - Local Test"
echo "========================================"

# Check if wash is installed
if ! command -v wash &> /dev/null; then
    echo "❌ wash CLI not found. Please install it first."
    exit 1
fi

# Check if WasmCloud is running
if ! wash get hosts &> /dev/null; then
    echo "⚠️  WasmCloud is not running. Starting..."
    wash up -d
    sleep 5
fi

echo "✓ WasmCloud is running"

# Build the provider
echo "📦 Building provider..."
cd ..
cargo build --release

# Build the component
echo "📦 Building component..."
cd wasmcloud-example/consumer-component
wash build

cd ..

echo "✓ Build complete"

# Deploy the application
echo "🚀 Deploying application..."
wash app deploy wadm.yaml

echo ""
echo "✅ Deployment complete!"
echo ""
echo "📝 Next steps:"
echo "  1. Start a TCP server: while true; do echo 'Test $(date)' | nc -l 8080; done"
echo "  2. Monitor NATS: nats sub 'wasmcloud.stream.messages.>'"
echo "  3. Check logs: wash logs"
echo ""
echo "To undeploy: wash app undeploy tcp-udp-stream-example"
