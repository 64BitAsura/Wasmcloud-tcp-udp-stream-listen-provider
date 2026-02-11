# Testing Guide

This document describes how to test the TCP/UDP Stream Listener Provider.

## Test Components

The project includes several test components:

1. **Unit Tests**: Basic functionality tests
2. **Integration Tests**: Test server for sending TCP/UDP messages
3. **Manual Tests**: Using netcat and other tools

## Running Unit Tests

```bash
# Run all tests
cargo test --workspace

# Run specific package tests
cargo test --package tcp-udp-stream-provider
cargo test --package stream-receiver
cargo test --package test-server

# Run with output
cargo test --workspace -- --nocapture
```

## Integration Testing with Test Server

The test server is a CLI tool that can send TCP and UDP messages to the provider.

### Basic Usage

```bash
# TCP messages (10 messages with 1 second interval)
./target/release/test-server tcp

# UDP messages (10 messages with 1 second interval)
./target/release/test-server udp

# Both TCP and UDP simultaneously
./target/release/test-server both
```

### Advanced Options

```bash
# Custom host and port
./target/release/test-server tcp --host 192.168.1.100 --port 9000

# Custom message count and interval
./target/release/test-server tcp --count 100 --interval 100

# Custom message prefix
./target/release/test-server tcp --message "Custom Message"

# Infinite messages (count=0)
./target/release/test-server tcp --count 0 --interval 500
```

### UDP Specific Options

```bash
# Send UDP to specific port
./target/release/test-server udp --port 8081

# Fast UDP messages
./target/release/test-server udp --count 1000 --interval 10
```

### Both Protocols

```bash
# Custom ports for both
./target/release/test-server both --tcp-port 8080 --udp-port 8081

# Custom count and interval
./target/release/test-server both --count 50 --interval 200
```

## Manual Testing with netcat

### TCP Testing

```bash
# Single line message
echo "Test message" | nc 127.0.0.1 8080

# Multi-line message
cat <<EOF | nc 127.0.0.1 8080
First line
Second line
Third line
EOF

# Interactive mode
nc 127.0.0.1 8080
# Type messages and press Enter
# Ctrl+C to exit

# Keep connection alive
nc 127.0.0.1 8080 < /dev/stdin
```

### UDP Testing

```bash
# Single UDP packet
echo "UDP test" | nc -u 127.0.0.1 8081

# Multiple packets
for i in {1..10}; do
  echo "UDP packet $i" | nc -u 127.0.0.1 8081
  sleep 0.5
done

# Interactive UDP
nc -u 127.0.0.1 8081
```

## Testing Message Parsing

The provider splits messages by newlines. Test this behavior:

```bash
# Multiple lines in one TCP message
echo -e "Line 1\nLine 2\nLine 3" | nc 127.0.0.1 8080

# Should see three separate log entries in provider output
```

## Performance Testing

### High Volume TCP

```bash
# Send many messages quickly
./target/release/test-server tcp --count 1000 --interval 10
```

### High Volume UDP

```bash
# Send many UDP packets
./target/release/test-server udp --count 1000 --interval 10
```

### Concurrent Connections

```bash
# Open multiple connections
for i in {1..10}; do
  ./target/release/test-server tcp --count 10 &
done
wait
```

## Load Testing

### Using ab (ApacheBench) style testing

```bash
# Continuous TCP stream
while true; do
  echo "Message $(date +%s.%N)" | nc 127.0.0.1 8080
done
```

### Parallel UDP senders

```bash
# Launch 10 parallel UDP senders
for i in {1..10}; do
  (
    for j in {1..100}; do
      echo "Sender $i Message $j" | nc -u 127.0.0.1 8081
      sleep 0.01
    done
  ) &
done
wait
```

## Expected Behavior

### Normal Operation

Provider should:
1. Accept all TCP connections
2. Receive all UDP packets
3. Parse each line as a separate message
4. Log each message with INFO level
5. Not crash or hang

### Edge Cases

Test these scenarios:

1. **Empty messages**
   ```bash
   echo "" | nc 127.0.0.1 8080  # Should be ignored
   ```

2. **Very long messages**
   ```bash
   python3 -c "print('A' * 65000)" | nc 127.0.0.1 8080
   ```

3. **Binary data** (should handle gracefully)
   ```bash
   dd if=/dev/urandom bs=1024 count=1 | nc 127.0.0.1 8080
   ```

4. **Rapid connection cycling**
   ```bash
   for i in {1..100}; do
     echo "Quick $i" | nc 127.0.0.1 8080
   done
   ```

## Monitoring Provider Health

### Check if provider is running

```bash
ps aux | grep tcp-udp-provider
```

### Check port usage

```bash
lsof -i :8080
lsof -i :8081
netstat -an | grep -E "(8080|8081)"
```

### Monitor resource usage

```bash
# CPU and memory
top -p $(pgrep tcp-udp-provider)

# Or with htop
htop -p $(pgrep tcp-udp-provider)
```

## Test Results

### Success Criteria

- ✅ Provider starts without errors
- ✅ TCP listener accepts connections
- ✅ UDP listener receives packets
- ✅ Messages are parsed correctly
- ✅ Each line is processed separately
- ✅ Provider logs all messages
- ✅ No memory leaks during extended runs
- ✅ Handles multiple concurrent connections

### Known Limitations

- Maximum message size: 64KB per read
- Lines longer than 64KB will be split
- Binary data is converted to UTF-8 lossy (invalid chars replaced with �)

## Automated Test Script

Create a test script `test.sh`:

```bash
#!/bin/bash
set -e

echo "Starting provider..."
./target/release/tcp-udp-provider &
PROVIDER_PID=$!
sleep 2

echo "Testing TCP..."
./target/release/test-server tcp --count 5 --interval 100

echo "Testing UDP..."
./target/release/test-server udp --count 5 --interval 100

echo "Testing both..."
./target/release/test-server both --count 3 --interval 100

echo "Stopping provider..."
kill $PROVIDER_PID

echo "All tests passed!"
```

Run it:
```bash
chmod +x test.sh
./test.sh
```

## Debugging

### Enable detailed logging

Set environment variable:
```bash
RUST_LOG=debug ./target/release/tcp-udp-provider
```

Or modify the code to use `debug` level.

### Capture network traffic

```bash
# TCP
sudo tcpdump -i lo port 8080 -X

# UDP
sudo tcpdump -i lo port 8081 -X
```

### Check for errors

```bash
# Run provider with error output
./target/release/tcp-udp-provider 2>&1 | tee provider.log
```

## Continuous Integration

For CI/CD pipelines:

```bash
# Quick smoke test
make build && make test

# Full integration test
./target/release/tcp-udp-provider &
PID=$!
sleep 2
./target/release/test-server both --count 10
kill $PID
```

## Reporting Issues

When reporting issues, include:

1. Provider version
2. Test command used
3. Expected vs actual behavior
4. Provider logs
5. System information (OS, Rust version)
6. Network configuration if relevant

Example:
```bash
# Gather debug info
./target/release/tcp-udp-provider --version
rustc --version
uname -a
./target/release/tcp-udp-provider 2>&1 | tee debug.log
```
