# Unix Domain Socket HTTP Server

A simple HTTP server implemented in Rust that communicates via Unix Domain Sockets. This server provides basic endpoints that return JSON responses.

## Features

- Lightweight HTTP server using Unix Domain Sockets
- JSON responses
- Two endpoints: `/say-hello` and `/say-goodbye`

## Prerequisites

- Rust (latest stable version)
- Unix-like operating system (Linux, macOS)

## Building and Running

1. Build the project:
```bash
cargo build --release
```

2. Run the server:
```bash
cargo run
```

The server will create a Unix Domain Socket at `/tmp/rust_server.sock`

## Testing with curl

You can test the endpoints using curl with the `--unix-socket` flag:

```bash
# Test the hello endpoint
curl --unix-socket /tmp/rust_server.sock http://localhost/say-hello

# Test the goodbye endpoint
curl --unix-socket /tmp/rust_server.sock http://localhost/say-goodbye
```

### Expected Responses

- `/say-hello` endpoint:
```json
{"message": "Hello, World!"}
```

- `/say-goodbye` endpoint:
```json
{"message": "Goodbye, World!"}
```

- Any other path will return a 404 response:
```json
{"message": "Not Found"}
```

## Note

The server automatically removes any existing socket file at `/tmp/rust_server.sock` on startup to prevent "Address already in use" errors.
