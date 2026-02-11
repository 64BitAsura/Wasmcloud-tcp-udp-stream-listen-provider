# TCP/UDP Stream Provider Examples

This directory contains examples of how to use the TCP/UDP Stream Provider with WasmCloud.

## Example 1: Basic TCP Stream Listener

This example demonstrates connecting to a TCP server and receiving messages.

### Configuration

```json
{
  "tcp": [
    {
      "host": "localhost",
      "port": 8080,
      "reconnect_delay_ms": 5000
    }
  ]
}
```

### Steps to Deploy

1. **Start a test TCP server** (for testing purposes):
   ```bash
   # Using netcat
   nc -l 8080
   # Or using Python
   python3 -m http.server 8080
   ```

2. **Start WasmCloud**:
   ```bash
   wash up
   ```

3. **Deploy the provider** (once packaged):
   ```bash
   wash start provider <provider-reference>
   ```

4. **Create a link to a component**:
   ```bash
   wash link put <component-id> <provider-id> \
     --link-name default \
     --values '{"tcp":[{"host":"localhost","port":8080}]}'
   ```

## Example 2: UDP Stream Listener

This example demonstrates receiving UDP messages.

### Configuration

```json
{
  "udp": [
    {
      "host": "localhost",
      "port": 9090
    }
  ]
}
```

### Steps to Deploy

1. **Start a test UDP server**:
   ```bash
   # Using netcat for UDP
   nc -u -l 9090
   ```

2. **Follow similar deployment steps as TCP example above**

## Example 3: Multiple Connections

This example shows how to connect to multiple TCP and UDP servers simultaneously.

### Configuration

```json
{
  "tcp": [
    {
      "host": "server1.example.com",
      "port": 8080,
      "reconnect_delay_ms": 5000
    },
    {
      "host": "server2.example.com",
      "port": 8081,
      "reconnect_delay_ms": 3000
    }
  ],
  "udp": [
    {
      "host": "udp-server.example.com",
      "port": 9090
    }
  ]
}
```

## Message Format

All messages received by the provider are forwarded to NATS with the following structure:

```json
{
  "source": "tcp://localhost:8080",
  "content": "Message content from the stream",
  "timestamp": 1234567890
}
```

Messages are published to NATS topics based on the source:
- Format: `wasmcloud.stream.messages.<sanitized_source>`
- Example: `wasmcloud.stream.messages.tcp_localhost_8080`

## Testing Locally

You can test the provider locally without deploying to WasmCloud:

1. **Run the provider in development mode**:
   ```bash
   cd provider
   RUST_LOG=info cargo run
   ```

2. **Set up test servers** in separate terminals:
   ```bash
   # Terminal 1: TCP server
   echo "Hello from TCP" | nc -l 8080
   
   # Terminal 2: UDP server
   echo "Hello from UDP" | nc -u -l 9090
   ```

## Monitoring Messages

To monitor messages flowing through NATS:

```bash
# Subscribe to all stream messages
nats sub "wasmcloud.stream.messages.>"

# Subscribe to specific source
nats sub "wasmcloud.stream.messages.tcp_localhost_8080"
```

## Troubleshooting

### Provider not connecting
- Check that the remote server is accessible
- Verify firewall rules allow the connection
- Check provider logs: `wash logs <provider-id>`

### Messages not appearing
- Verify NATS connection is active
- Check that component is subscribed to the correct topic
- Enable debug logging: `RUST_LOG=debug`

### Connection keeps dropping
- Increase `reconnect_delay_ms` in configuration
- Check network stability
- Verify remote server is stable
