#!/bin/bash
set -e

# Ensure wash and cargo are in PATH
export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track background PIDs for cleanup
PIDS=()
cleanup() {
    echo -e "${YELLOW}Cleaning up...${NC}"
    for pid in "${PIDS[@]}"; do
        kill "$pid" 2>/dev/null || true
        wait "$pid" 2>/dev/null || true
    done
    echo -e "${GREEN}Cleanup complete${NC}"
}
trap cleanup EXIT

echo -e "${GREEN}=== TCP/UDP Stream Provider Integration Test ===${NC}"
echo -e "${YELLOW}wash CLI version:${NC} $(wash --version 2>&1 || echo 'unknown')"
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

# Step 3: Build provider (wash v2 uses .wash/config.yaml for build configuration)
echo -e "${YELLOW}Step 3: Building provider...${NC}"
wash build
echo -e "${GREEN}Provider built${NC}"
echo ""

# Step 4: Build component
echo -e "${YELLOW}Step 4: Building test component...${NC}"
wash -C ./component build
echo -e "${GREEN}Component built${NC}"
echo ""

# Step 5: Run unit tests
echo -e "${YELLOW}Step 5: Running unit tests...${NC}"
cargo test
echo -e "${GREEN}Unit tests passed${NC}"
echo ""

# Step 6: Start TCP test server in background
TCP_PORT=9000
echo -e "${YELLOW}Step 6: Starting TCP test server on port ${TCP_PORT}...${NC}"
python3 tests/tcp_udp_server.py --protocol tcp --port "$TCP_PORT" &
TCP_PID=$!
PIDS+=("$TCP_PID")
sleep 2

if ! kill -0 $TCP_PID 2>/dev/null; then
    echo -e "${RED}Error: TCP test server failed to start${NC}"
    exit 1
fi
echo -e "${GREEN}TCP test server running (PID: $TCP_PID)${NC}"
echo ""

# Step 7: Run TCP integration test
echo -e "${YELLOW}Step 7: Running TCP stream integration test...${NC}"
TEST_TCP_PORT="$TCP_PORT" cargo test test_tcp_stream_connect -- --ignored
echo -e "${GREEN}TCP integration test passed${NC}"
echo ""

# Step 8: Start UDP test server in background
UDP_PORT=9001
echo -e "${YELLOW}Step 8: Starting UDP test server on port ${UDP_PORT}...${NC}"
python3 tests/tcp_udp_server.py --protocol udp --port "$UDP_PORT" &
UDP_PID=$!
PIDS+=("$UDP_PID")
sleep 2

if ! kill -0 $UDP_PID 2>/dev/null; then
    echo -e "${RED}Error: UDP test server failed to start${NC}"
    exit 1
fi
echo -e "${GREEN}UDP test server running (PID: $UDP_PID)${NC}"
echo ""

# Step 9: Run UDP integration test
echo -e "${YELLOW}Step 9: Running UDP stream integration test...${NC}"
TEST_UDP_PORT="$UDP_PORT" cargo test test_udp_stream_connect -- --ignored
echo -e "${GREEN}UDP integration test passed${NC}"
echo ""

echo -e "${GREEN}=== All integration tests passed ===${NC}"
