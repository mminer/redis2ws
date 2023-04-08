use futures_util::{SinkExt, StreamExt};
use redis::aio::PubSub;
use redis::IntoConnectionInfo;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg_matches = clap::command!()
        .args(&[
            clap::Arg::new("redis_addr")
                .default_value("localhost:6379")
                .help("Redis server address")
                .long("redis")
                .value_name("URL"),

            clap::Arg::new("redis_channel")
                .help("Redis PubSub channel")
                .required(true)
                .value_name("CHANNEL"),

            clap::Arg::new("ws_addr")
                .default_value("localhost:8080")
                .help("WebSocket server address")
                .long("ws")
                .value_name("URL"),
        ])
        .get_matches();

    let (sender, receiver) = watch::channel("".to_string());

    // Listen for messages from Redis channel.
    let addr = arg_matches.get_one::<String>("redis_addr").unwrap().to_owned();
    let connection_info = format!("redis://{addr}").into_connection_info()?;
    let channel = arg_matches.get_one::<String>("redis_channel").unwrap();
    let client = redis::Client::open(connection_info).unwrap();
    let mut pubsub = client.get_async_connection().await?.into_pubsub();
    pubsub.subscribe(channel).await?;
    tokio::spawn(handle_redis_pubsub(pubsub, sender));
    println!("Subscribed to Redis Pub/Sub channel: {channel}");

    // Handle WebSocket connections.
    let addr = arg_matches.get_one::<String>("ws_addr").unwrap();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server listening on: {addr}");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_websocket_connection(stream, receiver.clone()));
    }

    Ok(())
}

async fn handle_redis_pubsub(mut pubsub: PubSub, sender: watch::Sender<String>) {
    while let Some(message) = pubsub.on_message().next().await {
        match message.get_payload::<String>() {
            Ok(value) => {
                sender.send_replace(value);
            },
            Err(e) => {
                eprintln!("Error getting Redis pubsub payload: {e}");
            },
        }
    }
}

async fn handle_websocket_connection(stream: TcpStream, mut receiver: watch::Receiver<String>) {
    let addr = stream.peer_addr().expect("connected stream lacks peer address");

    let mut ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("WebSocket handshake failed");

    println!("New WebSocket connection: {addr}");

    loop {
        let value = receiver.borrow_and_update().clone();
        let message = tungstenite::Message::Text(value);

        if let Err(e) = ws_stream.send(message).await {
            eprintln!("Error sending to WebSocket client: {e}");
            break;
        }

        receiver.changed().await.unwrap();
    }

    println!("{addr} disconnected");
}
