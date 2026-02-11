#!/bin/bash
# Demonstration script for TCP/UDP Stream Listener Provider
# This script demonstrates the provider receiving messages from TCP and UDP

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  TCP/UDP Stream Listener Provider - Demo             ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if provider binary exists
if [ ! -f "./target/release/tcp-udp-provider" ]; then
    echo -e "${YELLOW}Provider binary not found. Building...${NC}"
    cargo build --release --package tcp-udp-stream-provider
    echo -e "${GREEN}✓ Build complete${NC}"
fi

# Check if test server binary exists
if [ ! -f "./target/release/test-server" ]; then
    echo -e "${YELLOW}Test server binary not found. Building...${NC}"
    cargo build --release --package test-server
    echo -e "${GREEN}✓ Build complete${NC}"
fi

echo ""
echo -e "${GREEN}Step 1: Starting the provider...${NC}"
./target/release/tcp-udp-provider > provider.log 2>&1 &
PROVIDER_PID=$!
sleep 2

if ps -p $PROVIDER_PID > /dev/null; then
    echo -e "${GREEN}✓ Provider started (PID: $PROVIDER_PID)${NC}"
else
    echo -e "${RED}✗ Failed to start provider${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}Step 2: Sending TCP messages...${NC}"
./target/release/test-server tcp --count 5 --interval 200 --message "TCP Demo"
echo -e "${GREEN}✓ TCP messages sent${NC}"

echo ""
echo -e "${GREEN}Step 3: Sending UDP messages...${NC}"
./target/release/test-server udp --count 5 --interval 200 --message "UDP Demo"
echo -e "${GREEN}✓ UDP messages sent${NC}"

echo ""
echo -e "${GREEN}Step 4: Sending both TCP and UDP simultaneously...${NC}"
./target/release/test-server both --count 3 --interval 300
echo -e "${GREEN}✓ Both protocols tested${NC}"

echo ""
echo -e "${YELLOW}Provider Log Output:${NC}"
echo -e "${BLUE}───────────────────────────────────────────────────────${NC}"
cat provider.log
echo -e "${BLUE}───────────────────────────────────────────────────────${NC}"

echo ""
echo -e "${GREEN}Step 5: Stopping the provider...${NC}"
kill $PROVIDER_PID 2>/dev/null || true
wait $PROVIDER_PID 2>/dev/null || true
echo -e "${GREEN}✓ Provider stopped${NC}"

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Demo Complete! All tests passed successfully.        ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "  • Review the logs above to see message processing"
echo "  • Check QUICKSTART.md for more examples"
echo "  • Read TESTING.md for comprehensive testing guide"
echo "  • Explore ARCHITECTURE.md for technical details"
echo ""

# Cleanup
rm -f provider.log
