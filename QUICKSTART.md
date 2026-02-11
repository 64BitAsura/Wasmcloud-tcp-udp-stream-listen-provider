# Quick Start Guide

This guide will help you get started with the TCP/UDP Stream Listener Provider quickly.

## Prerequisites

- Rust 1.70+ with `wasm32-wasip1` target
- Make (optional, for using Makefile commands)

## Quick Start (5 minutes)

### 1. Build Everything

```bash
make build
```

Or build individually:
```bash
./build-provider.sh
./build-component.sh
./build-test-server.sh
```

### 2. Start the Provider

In one terminal:
```bash
./target/release/tcp-udp-provider
```

You should see:
```
INFO tcp_udp_stream_provider: Starting TCP/UDP Stream Provider
INFO tcp_udp_stream_provider: TCP listener running on 0.0.0.0:8080
INFO tcp_udp_stream_provider: UDP listener running on 0.0.0.0:8081
INFO tcp_udp_stream_provider: TCP/UDP Stream Provider started successfully
```

### 3. Send Test Messages

In another terminal, send TCP messages:
```bash
./target/release/test-server tcp --count 5
```

Or send UDP messages:
```bash
./target/release/test-server udp --count 5
```

Or send both:
```bash
./target/release/test-server both --count 5
```

### 4. Verify Message Reception

Check the provider terminal output. You should see messages like:
```
INFO tcp_udp_stream_provider: Received TCP message from 127.0.0.1:xxxxx: TCP Message #1
INFO tcp_udp_stream_provider: Processing TCP line: TCP Message #1
INFO tcp_udp_stream_provider: Received UDP message from 127.0.0.1:xxxxx: UDP Message #1
INFO tcp_udp_stream_provider: Processing UDP line: UDP Message #1
```

## Testing with netcat

You can also use standard Unix tools:

### TCP
```bash
# Single message
echo "Hello TCP" | nc 127.0.0.1 8080

# Interactive
nc 127.0.0.1 8080
```

### UDP
```bash
# Single message
echo "Hello UDP" | nc -u 127.0.0.1 8081

# Interactive
nc -u 127.0.0.1 8081
```

## Using Make Commands

The project includes convenient make targets:

```bash
# Build all components
make build

# Run tests
make test

# Send TCP test messages
make run-test-tcp

# Send UDP test messages
make run-test-udp

# Send both TCP and UDP
make run-test-both

# Clean build artifacts
make clean

# Show all available commands
make help
```

## Custom Ports

To run the provider on custom ports, modify the code or use environment variables (future feature).

Currently, default ports are:
- TCP: 8080
- UDP: 8081

## Next Steps

1. Review the [README.md](README.md) for complete documentation
2. Check [examples/](examples/) for configuration examples
3. Explore the component implementation in [component/src/lib.rs](component/src/lib.rs)
4. Customize the provider for your use case

## Troubleshooting

### Port already in use
```bash
# Check what's using the port
lsof -i :8080
lsof -i :8081

# Kill the process
kill <PID>
```

### Build errors
```bash
# Clean and rebuild
cargo clean
make build
```

### Can't connect to provider
- Ensure the provider is running
- Check firewall settings
- Verify the correct ports are being used

## Examples

### Send custom messages
```bash
# TCP with custom message
echo "My custom TCP message" | nc 127.0.0.1 8080

# UDP with custom message  
echo "My custom UDP message" | nc -u 127.0.0.1 8081
```

### Multi-line messages
```bash
# Each line is processed separately
cat <<EOF | nc 127.0.0.1 8080
Line 1
Line 2
Line 3
EOF
```

### Continuous stream
```bash
# Send messages every second
while true; do 
  echo "Stream message $(date)" | nc 127.0.0.1 8080
  sleep 1
done
```

## Support

For issues or questions, please open an issue on GitHub.
