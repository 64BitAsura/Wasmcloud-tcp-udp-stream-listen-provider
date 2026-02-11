# TCP/UDP Stream Provider - WasmCloud Example

This directory contains a complete example demonstrating the TCP/UDP Stream Provider with a consumer component.

## Structure

- `consumer-component/` - A WasmCloud component that receives stream messages
- `wadm.yaml` - WasmCloud Application Deployment Manager manifest
- `Makefile` - Convenience commands for building and deploying

## Quick Start

### Prerequisites

- wash CLI installed
- WasmCloud running (`wash up`)
- A TCP or UDP server for testing

### Build and Deploy

```bash
# Build both provider and component
make build

# Deploy to WasmCloud
make deploy

# Monitor logs
wash app list
wash logs
```

### Test with Mock Server

In a separate terminal, start a mock TCP server:

```bash
# Simple TCP server that sends periodic messages
while true; do echo "Message at $(date)" | nc -l 8080; sleep 1; done
```

Monitor messages in NATS:

```bash
nats sub "wasmcloud.stream.messages.>"
```

## Configuration

The example is configured to connect to `localhost:8080` for TCP. Edit `wadm.yaml` to change:

- Connection host and port
- Protocol (TCP or UDP)
- Reconnection settings

## Cleanup

```bash
# Undeploy the application
make undeploy

# Stop WasmCloud
wash down
```

## See Also

- [QUICKSTART.md](../QUICKSTART.md) - Quick start guide
- [README.md](../README.md) - Main documentation
