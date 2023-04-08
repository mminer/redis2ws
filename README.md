# redis2ws (Redis to WebSockets)

This demonstrates a Rust WebSockets server that broadcasts Redis Pub/Sub
messages to connected clients. Glues together the
[redis](https://docs.rs/redis/latest/redis/) and
[tungstenite](https://docs.rs/tungstenite/latest/tungstenite/) crates.

## Usage

1. Start a Redis server
1. `cargo run foo`
1. Connect to ws://localhost:8080
   ([wscat](https://github.com/websockets/wscat) works well for testing)
1. Publish a message to the Redis `foo` channel
