#!/usr/bin/env python3
"""
Simple TCP/UDP test server that sends periodic messages.
Used for testing the wasmCloud TCP/UDP stream listen provider.
"""

import argparse
import asyncio
import json
import socket
import sys
from datetime import datetime


async def handle_tcp_client(reader, writer):
    """Handle a single TCP client connection."""
    addr = writer.get_extra_info("peername")
    print(f"TCP client connected from {addr}")

    message_count = 0
    try:
        while True:
            message_count += 1

            msg = json.dumps({
                "type": "test",
                "count": message_count,
                "timestamp": datetime.utcnow().isoformat(),
                "message": f"Test message #{message_count}",
            })

            writer.write((msg + "\n").encode())
            await writer.drain()
            print(f"Sent to TCP client {addr}: {msg}")

            await asyncio.sleep(3)

    except (ConnectionResetError, BrokenPipeError) as e:
        print(f"TCP client {addr} disconnected: {e}")
    finally:
        writer.close()
        await writer.wait_closed()


async def run_tcp_server(host, port):
    """Start a TCP server."""
    server = await asyncio.start_server(handle_tcp_client, host, port)
    addrs = ", ".join(str(s.getsockname()) for s in server.sockets)
    print(f"TCP test server listening on {addrs}")
    print("Press Ctrl+C to stop")
    print("-" * 50)

    async with server:
        await server.serve_forever()


async def run_udp_server(host, port):
    """Start a UDP server that sends periodic messages to connected clients."""

    class UdpServerProtocol(asyncio.DatagramProtocol):
        def __init__(self):
            self.transport = None
            self.clients = set()

        def connection_made(self, transport):
            self.transport = transport

        def datagram_received(self, data, addr):
            print(f"UDP received from {addr}: {data.decode(errors='replace')}")
            self.clients.add(addr)

    loop = asyncio.get_running_loop()
    transport, protocol = await loop.create_datagram_endpoint(
        UdpServerProtocol, local_addr=(host, port)
    )

    print(f"UDP test server listening on {host}:{port}")
    print("Press Ctrl+C to stop")
    print("-" * 50)

    # Send periodic messages to known clients
    message_count = 0
    try:
        while True:
            message_count += 1

            msg = json.dumps({
                "type": "test",
                "count": message_count,
                "timestamp": datetime.utcnow().isoformat(),
                "message": f"Test message #{message_count}",
            })

            for client_addr in list(protocol.clients):
                transport.sendto((msg + "\n").encode(), client_addr)
                print(f"Sent to UDP client {client_addr}: {msg}")

            await asyncio.sleep(3)
    finally:
        transport.close()


async def main():
    parser = argparse.ArgumentParser(description="TCP/UDP test server")
    parser.add_argument(
        "--protocol",
        choices=["tcp", "udp"],
        default="tcp",
        help="Protocol to use (default: tcp)",
    )
    parser.add_argument(
        "--host", default="127.0.0.1", help="Host to bind to (default: 127.0.0.1)"
    )
    parser.add_argument(
        "--port", type=int, default=9000, help="Port to bind to (default: 9000)"
    )
    args = parser.parse_args()

    print(f"Starting {args.protocol.upper()} test server on {args.host}:{args.port}")

    if args.protocol == "tcp":
        await run_tcp_server(args.host, args.port)
    else:
        await run_udp_server(args.host, args.port)


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nServer stopped")
