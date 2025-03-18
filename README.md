# Simple TCP Server

A lightweight TCP server implementation in Rust that handles client connections concurrently.

## Features

- Listens for TCP connections on localhost (127.0.0.1) port 8080
- Handles multiple client connections concurrently using threads
- Echoes a simple greeting message back to clients
- Basic error handling for connection issues

## Prerequisites

- Rust programming language (latest stable version recommended)
- Cargo package manager

## Installation

1. Clone this repository or copy the code into a new Rust project:

```bash
cargo new tcp_server
cd tcp_server
```

2. Replace the contents of `src/main.rs` with the provided code.

3. Build the project:

```bash
cargo build --release
```

## Usage

1. Start the server:

```bash
cargo run
```

Or run the compiled binary directly:

```bash
./target/release/tcp_server
```

2. The server will start and display: "Server listening on 127.0.0.1:8080"

3. Connect to the server using a TCP client like netcat:

```bash
nc 127.0.0.1 8080
```

4. Any message sent to the server will be displayed in the server console, and the server will respond with "Hello, client!"

## Code Explanation

- The server creates a TCP listener bound to 127.0.0.1:8080
- For each incoming connection, it spawns a new thread to handle the client
- The `handle_client` function reads data from the client and sends a response
- Basic error handling is implemented for connection failures

## Limitations

- Currently only responds with a fixed message regardless of input
- No proper HTTP protocol implementation
- No configuration options for address or port
- No graceful shutdown mechanism

## Possible Improvements

- Implement proper protocol handling (HTTP, custom protocol, etc.)
- Add configuration options (port, address, response message)
- Implement a thread pool instead of spawning unlimited threads
- Add logging functionality
- Add tests
- Implement proper error handling and recovery
