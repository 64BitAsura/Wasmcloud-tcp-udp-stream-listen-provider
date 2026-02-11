# Quick Start Guide

This guide will help you get the TCP/UDP Stream Provider running quickly for local testing.

## Prerequisites

- Rust (latest stable): https://rustup.rs/
- wash CLI installed:
  ```bash
  curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | sudo bash
  sudo apt install wash
  ```

## Quick Test (No WasmCloud Required)

### Test TCP Connection

1. **Start a mock TCP server** (in terminal 1):
   ```bash
   # Using netcat to create a simple TCP server
   while true; do echo "Hello from TCP $(date)" | nc -l 8080; done
   ```

2. **Build and run the provider** (in terminal 2):
   ```bash
   cd provider
   cargo build --release
   
   # The provider needs host data which is normally provided by wasmCloud
   # For standalone testing, you would need to set environment variables
   # or use the wasmCloud runtime
   ```

### Test UDP Connection

1. **Start a mock UDP server** (in terminal 1):
   ```bash
   # Using netcat for UDP
   nc -u -l 9090
   ```

2. **Send test data**:
   ```bash
   echo "Test UDP message" | nc -u localhost 9090
   ```

## Run with WasmCloud

### 1. Start WasmCloud

```bash
wash up
```

This starts:
- NATS server
- WasmCloud host
- wadm (application deployment manager)

### 2. Build the Provider

```bash
cd provider
cargo build --release
```

### 3. Create Provider Archive (PAR)

```bash
wash par create \
  --capid wasmcloud:stream \
  --vendor tcp-udp \
  --name "TCP/UDP Stream Provider" \
  --binary target/release/tcp-udp-stream-provider \
  --destination ../build/tcp-udp-stream-provider.par.gz
```

### 4. Start the Provider in WasmCloud

```bash
wash start provider file://$(pwd)/../build/tcp-udp-stream-provider.par.gz
```

### 5. Link to a Component

First, you need a component that will receive the stream messages. Create a link:

```bash
wash link put <component-id> <provider-id> \
  --link-name default \
  --values '{
    "tcp": [
      {"host": "localhost", "port": 8080, "reconnect_delay_ms": 5000}
    ]
  }'
```

## Monitor Messages

To see messages flowing through NATS:

```bash
# Subscribe to all stream messages
nats sub "wasmcloud.stream.messages.>"

# Subscribe to specific source
nats sub "wasmcloud.stream.messages.tcp_localhost_8080"
```

## Test End-to-End

1. **Start a TCP server** (terminal 1):
   ```bash
   while true; do echo "Message $(date)" | nc -l 8080; sleep 1; done
   ```

2. **Monitor NATS** (terminal 2):
   ```bash
   nats sub "wasmcloud.stream.messages.>"
   ```

3. **Watch the messages** flowing from TCP server → Provider → NATS → Component

## Configuration Examples

### Multiple TCP Connections

```json
{
  "tcp": [
    {"host": "server1.example.com", "port": 8080},
    {"host": "server2.example.com", "port": 8081}
  ]
}
```

### Mixed TCP and UDP

```json
{
  "tcp": [
    {"host": "metrics.example.com", "port": 8080, "reconnect_delay_ms": 3000}
  ],
  "udp": [
    {"host": "logs.example.com", "port": 9090}
  ]
}
```

## Troubleshooting

### Provider won't start
- Check that NATS is running: `wash get hosts`
- Check provider logs: `wash logs <provider-id>`

### No messages appearing
- Verify TCP/UDP server is accessible: `nc -zv localhost 8080`
- Check NATS subscription: `nats sub "wasmcloud.stream.messages.>"`
- Enable debug logging: `RUST_LOG=debug`

### Connection keeps dropping
- Check network connectivity
- Increase `reconnect_delay_ms` in configuration
- Verify firewall rules

## Next Steps

- See [README.md](../README.md) for full documentation
- Check [examples/](../examples/) for usage examples
- Review [Agents.md](../Agents.md) for architecture details

## Clean Up

```bash
# Stop wasmCloud
wash down

# Remove test servers
# Press Ctrl+C in the terminals running netcat
```
