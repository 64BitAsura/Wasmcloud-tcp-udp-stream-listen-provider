#!/bin/bash
set -e

# Ensure wash and cargo are in PATH
export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"

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

echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo -e "${YELLOW}Cleaning up...${NC}"

    # Stop TCP test server
    if [ ! -z "$TCP_SERVER_PID" ]; then
        kill $TCP_SERVER_PID 2>/dev/null || true
        echo "✓ Stopped TCP test server"
    fi

    # Stop wasmCloud
    wash down 2>/dev/null || true
    if [ ! -z "$WASH_PID" ]; then
        kill $WASH_PID 2>/dev/null || true
    fi
    echo "✓ Stopped wasmCloud"

    echo -e "${GREEN}Cleanup complete${NC}"
}

# Set up trap to cleanup on exit
trap cleanup EXIT

# Step 1: Format check
echo -e "${YELLOW}Step 1: Checking formatting...${NC}"
cargo fmt --all -- --check
echo -e "${GREEN}✓ Formatting OK${NC}"
echo ""

# Step 2: Clippy
echo -e "${YELLOW}Step 2: Running clippy...${NC}"
cargo clippy --release -- -D warnings
echo -e "${GREEN}✓ Clippy passed${NC}"
echo ""

# Step 3: Build provider
echo -e "${YELLOW}Step 3: Building provider...${NC}"
wash build 2>&1 | grep -E "(Compiling|Finished|error|Built)" || true

# Find the built provider archive
PROVIDER_PATH=$(find build -name "*.par.gz" 2>/dev/null | head -1)
if [ -z "$PROVIDER_PATH" ]; then
    echo -e "${RED}Error: Provider build failed - no .par.gz file found${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Provider built: $PROVIDER_PATH${NC}"
echo ""

# Step 4: Build component
echo -e "${YELLOW}Step 4: Building test component...${NC}"
wash build -p ./component 2>&1 | grep -E "(Compiling|Finished|error|Built)" || true

COMPONENT_PATH=$(find component/build -name "*.wasm" 2>/dev/null | head -1)
if [ -z "$COMPONENT_PATH" ]; then
    echo -e "${RED}Error: Component build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Component built: $COMPONENT_PATH${NC}"
echo ""

# Step 5: Run unit tests
echo -e "${YELLOW}Step 5: Running unit tests...${NC}"
cargo test
echo -e "${GREEN}✓ Unit tests passed${NC}"
echo ""

# Step 6: Start TCP test server
TCP_PORT=9000
echo -e "${YELLOW}Step 6: Starting TCP test server on port ${TCP_PORT}...${NC}"
python3 tests/tcp_udp_server.py --protocol tcp --port "$TCP_PORT" > /tmp/tcp_server.log 2>&1 &
TCP_SERVER_PID=$!
sleep 2

if ! kill -0 $TCP_SERVER_PID 2>/dev/null; then
    echo -e "${RED}Error: TCP test server failed to start${NC}"
    cat /tmp/tcp_server.log
    exit 1
fi

echo -e "${GREEN}✓ TCP test server started (PID: $TCP_SERVER_PID)${NC}"
echo "  Listening on 127.0.0.1:${TCP_PORT}"
echo ""

# Step 7: Start wasmCloud host
echo -e "${YELLOW}Step 7: Starting wasmCloud host...${NC}"

WASMCLOUD_LOG="/tmp/wasmcloud_host.log"
wash up > "$WASMCLOUD_LOG" 2>&1 &
WASH_PID=$!

# Wait for host to be ready
echo "Waiting for host to be ready..."
for i in {1..30}; do
    if wash get hosts 2>/dev/null | grep -qE "[A-Z0-9]{56}"; then
        break
    fi
    sleep 1
done

echo -e "${GREEN}✓ wasmCloud host started${NC}"
echo ""

# Step 8: Deploy provider
echo -e "${YELLOW}Step 8: Deploying provider...${NC}"
wash start provider "file://./$PROVIDER_PATH" tcp-udp-stream-provider --timeout-ms 30000 2>&1 || true
sleep 5

if wash get inventory 2>&1 | grep -q "tcp-udp-stream-provider"; then
    echo -e "${GREEN}✓ Provider deployed and running${NC}"
else
    echo -e "${RED}Error: Provider failed to start${NC}"
    wash get inventory 2>&1
    exit 1
fi
echo ""

# Step 9: Deploy component
echo -e "${YELLOW}Step 9: Deploying component...${NC}"
wash start component "file://./$COMPONENT_PATH" test-component --timeout-ms 30000 2>&1 || true
sleep 3

if wash get inventory 2>&1 | grep -q "test-component"; then
    echo -e "${GREEN}✓ Component deployed and running${NC}"
else
    echo -e "${RED}Error: Component failed to start${NC}"
    wash get inventory 2>&1
    exit 1
fi
echo ""

# Step 10: Create config and link
echo -e "${YELLOW}Step 10: Creating link between component and provider...${NC}"

wash config put stream-config \
  protocol=tcp \
  host=127.0.0.1 \
  port="$TCP_PORT"

wash link put test-component tcp-udp-stream-provider \
  wasmcloud messaging \
  --interface handler \
  --target-config stream-config

sleep 2
echo -e "${GREEN}✓ Link created${NC}"
echo ""

# Step 11: Monitor logs for messages
echo -e "${GREEN}=== Monitoring wasmCloud Logs ===${NC}"
echo "Waiting 30 seconds for messages to flow..."
echo ""

for i in {1..30}; do
    echo -ne "\rTime: ${i}s / 30s  "
    sleep 1
done

echo ""
echo ""

# Step 12: Analyze logs from the wasmCloud host output
echo -e "${GREEN}=== Test Results ===${NC}"
echo ""

PROVIDER_CONNECTED=$(grep -c "TCP stream connected" "$WASMCLOUD_LOG" 2>/dev/null || echo "0")
MESSAGES_SENT=$(grep -c "Message successfully sent to component" "$WASMCLOUD_LOG" 2>/dev/null || echo "0")
COMPONENT_RECEIVED=$(grep -c "Received message" "$WASMCLOUD_LOG" 2>/dev/null || echo "0")

echo "Provider — TCP stream connected:      $PROVIDER_CONNECTED"
echo "Provider — Messages sent to component: $MESSAGES_SENT"
echo "Component — Messages received:         $COMPONENT_RECEIVED"
echo ""

if [ "$PROVIDER_CONNECTED" -gt "0" ] && [ "$MESSAGES_SENT" -gt "0" ] && [ "$COMPONENT_RECEIVED" -gt "0" ]; then
    echo -e "${GREEN}✓ Integration test PASSED${NC}"
    echo ""
    echo "The provider successfully:"
    echo "  - Connected to the TCP test server"
    echo "  - Received messages from the server"
    echo "  - Forwarded messages to the component via wRPC"
    echo "  - Component processed the messages"
    exit 0
else
    echo -e "${RED}✗ Integration test FAILED${NC}"
    echo ""
    echo "Last 50 lines of host logs:"
    tail -50 "$WASMCLOUD_LOG" 2>/dev/null || echo "(no logs available)"
    exit 1
fi
