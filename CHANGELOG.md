# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-11

### Added
- Initial scaffolding generated via `wash new provider --template-name messaging-nats`
- Adapted scaffolding for TCP/UDP stream listen provider
- TCP stream client: connects to remote server, reads newline-delimited ASCII messages
- UDP datagram client: binds local socket, connects to remote server, receives ASCII datagrams
- Connection configuration module with protocol, host, port, subscriptions
- Per-component stream management with automatic cleanup on unlink/shutdown
- WIT interface definitions (`wasmcloud:messaging@0.2.0`)
- wasmcloud.toml provider metadata
- local.wadm.yaml deployment manifest
- CI/CD pipeline (GitHub Actions: fmt, clippy, build, test, security audit, release)
- Unit tests for configuration parsing, merging, defaults
- Unit tests for provider creation, shutdown, publish/request error paths
- Integration tests for build verification and config parsing
- Example: basic_usage.rs
- Documentation: README, Agents.md, CHANGELOG, CONTRIBUTING, SECURITY, TECHNICAL, CONFIG, QUICKSTART, IMPLEMENTATION

### Known Limitations
- Reply-back (publish/request to remote server) is deferred
- No TLS support
- No automatic reconnection
- ASCII-only (non-UTF-8 UDP datagrams are skipped)

[0.1.0]: https://github.com/64BitAsura/Wasmcloud-tcp-udp-stream-listen-provider/releases/tag/v0.1.0
