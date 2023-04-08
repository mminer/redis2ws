# redis2ws (Redis to WebSockets)

This demonstrates a Rust WebSockets server that broadcasts Redis Pub/Sub
messages to connected clients. Glues together the
[redis](https://docs.rs/redis/latest/redis/) and
[tungstenite](https://docs.rs/tungstenite/latest/tungstenite/) crates.

## Usage

1. Start a Redis server
1. Run `cargo run foo`
1. Connect to ws://localhost:8080
   ([wscat](https://github.com/websockets/wscat) works well for testing)
1. Publish a message to the Redis `foo` channel

```
Usage: redis2ws [OPTIONS] <CHANNEL>

Arguments:
  <CHANNEL>  Redis PubSub channel

Options:
      --redis <URL>  Redis server address [default: localhost:6379]
      --ws <URL>     WebSocket server address [default: localhost:8080]
  -h, --help         Print help
  -V, --version      Print version
```
