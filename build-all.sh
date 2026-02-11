#!/bin/bash
set -e

echo "Building all components..."

./build-provider.sh
./build-component.sh
./build-test-server.sh

echo ""
echo "All builds complete!"
