# Quick Start Guide

Get started with the TCP/UDP Stream Listen Provider in 5 minutes.

## Prerequisites

- Rust 1.70+
- [wash CLI](https://wasmcloud.com/docs/installation)

## Install wash CLI

```bash
curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | sudo bash
sudo apt install wash
```

## Build

```bash
git clone https://github.com/64BitAsura/Wasmcloud-tcp-udp-stream-listen-provider.git
cd Wasmcloud-tcp-udp-stream-listen-provider
cargo build --release
```

## Run Tests

```bash
cargo test
```

## Test with a Local TCP Server

### 1. Start a simple TCP echo server

```bash
# In terminal 1 — start a TCP server that sends lines
while true; do echo "Hello from TCP $(date)"; sleep 2; done | nc -lk 9000
```

### 2. Deploy with wasmCloud

```bash
# In terminal 2
wash up -d
wash app deploy local.wadm.yaml
```

### 3. Verify

Check wasmCloud logs for messages being forwarded:

```bash
wash get inventory
```

## Test with a Local UDP Server

### 1. Start a simple UDP sender

```bash
# Send UDP datagrams every 2 seconds
while true; do echo "Hello from UDP $(date)" | nc -u 127.0.0.1 5555; sleep 2; done
```

### 2. Update wadm.yaml

Change the provider config:

```yaml
config:
  - name: provider-config
    properties:
      protocol: udp
      host: "127.0.0.1"
      port: "5555"
```

### 3. Deploy

```bash
wash app deploy local.wadm.yaml
```

## Configuration Options

| Property   | Values      | Default     |
|------------|-------------|-------------|
| protocol   | tcp, udp    | tcp         |
| host       | hostname/IP | 127.0.0.1  |
| port       | 1-65535     | 9000        |

## Enable Debug Logging

```bash
RUST_LOG=debug wash up
```

## Next Steps

- [CONFIG.md](CONFIG.md) — Full configuration reference
- [TECHNICAL.md](TECHNICAL.md) — Architecture and design
- [CONTRIBUTING.md](CONTRIBUTING.md) — Development setup
