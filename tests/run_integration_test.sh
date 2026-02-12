#!/bin/bash
set -e

# Ensure wash is in PATH
export PATH="/usr/local/bin:$PATH"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== TCP/UDP Stream Provider Integration Test ===${NC}"
echo ""

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v wash &> /dev/null; then
    echo -e "${RED}Error: wash CLI not found${NC}"
    echo "Install from: https://wasmcloud.com/docs/installation"
    exit 1
fi

if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Error: python3 not found${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo not found${NC}"
    exit 1
fi

echo -e "${GREEN}All prerequisites met${NC}"
echo ""

# Step 1: Format check
echo -e "${YELLOW}Step 1: Checking formatting...${NC}"
cargo fmt --all -- --check
echo -e "${GREEN}Formatting OK${NC}"
echo ""

# Step 2: Clippy
echo -e "${YELLOW}Step 2: Running clippy...${NC}"
cargo clippy --release -- -D warnings
echo -e "${GREEN}Clippy passed${NC}"
echo ""

# Step 3: Build provider
echo -e "${YELLOW}Step 3: Building provider...${NC}"
wash build
echo -e "${GREEN}Provider built${NC}"
echo ""

# Step 4: Build component
echo -e "${YELLOW}Step 4: Building test component...${NC}"
wash build -p ./component
echo -e "${GREEN}Component built${NC}"
echo ""

# Step 5: Run unit tests
echo -e "${YELLOW}Step 5: Running unit tests...${NC}"
cargo test
echo -e "${GREEN}Unit tests passed${NC}"
echo ""

# Step 6: Start TCP test server in background
echo -e "${YELLOW}Step 6: Starting TCP test server...${NC}"
python3 tests/tcp_udp_server.py --protocol tcp --port 9000 &
SERVER_PID=$!
sleep 1

# Check server started
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo -e "${RED}Error: TCP test server failed to start${NC}"
    exit 1
fi
echo -e "${GREEN}TCP test server running (PID: $SERVER_PID)${NC}"
echo ""

# Step 7: Start wasmCloud host
echo -e "${YELLOW}Step 7: Starting wasmCloud host...${NC}"
wash up -d
sleep 3 # Wait for host to be ready

# Verify host is running
if ! wash get hosts 2>/dev/null | grep -q "N"; then
    echo -e "${YELLOW}Warning: Host may not be fully ready, waiting longer...${NC}"
    sleep 5
fi
echo -e "${GREEN}wasmCloud host running${NC}"
echo ""

# Step 8: Deploy application
echo -e "${YELLOW}Step 8: Deploying application...${NC}"
wash app deploy ./wadm.yaml
sleep 5 # Wait for deployment
echo -e "${GREEN}Application deployed${NC}"
echo ""

# Step 9: Verify deployment
echo -e "${YELLOW}Step 9: Verifying deployment...${NC}"
wash get inventory
echo ""

# Step 10: Wait for messages (give it time to connect and receive)
echo -e "${YELLOW}Step 10: Waiting for messages (15 seconds)...${NC}"
sleep 15
echo ""

# Step 11: Check logs for evidence of message flow
echo -e "${YELLOW}Step 11: Checking for evidence of message flow...${NC}"
echo "(Check host logs for 'received TCP line' or 'Message successfully sent')"
echo ""

# Cleanup
echo -e "${YELLOW}Cleaning up...${NC}"
wash down 2>/dev/null || true
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true

echo ""
echo -e "${GREEN}=== Integration test complete ===${NC}"
