use axum::{extract::ws::Message, extract::WebSocketUpgrade, response::IntoResponse, Router};
use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Failed to create producer");

    let app = Router::new().route(
        "/ws",
        axum::routing::get({
            let producer = producer.clone();
            move |ws| ws_handler(ws, producer)
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Game server running on :3000");
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, producer: FutureProducer) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, producer))
}

async fn handle_socket(socket: axum::extract::ws::WebSocket, producer: FutureProducer) {
    let (mut _tx, mut rx) = socket.split();

    while let Some(Ok(msg)) = rx.next().await {
        if let Message::Text(text) = msg {
            let topic = detect_topic(&text);
            let _ = producer
                .send(
                    FutureRecord::to(topic).payload(&text).key(""),
                    Duration::from_secs(5),
                )
                .await;
            println!("Sent to {}: {}", topic, text);
        }
    }
}

fn detect_topic(text: &str) -> &'static str {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
        match value.get("type").and_then(|t| t.as_str()) {
            Some("Move") | Some("Attack") | Some("UseItem") => "player-actions",
            Some("Start") | Some("Kill") | Some("Score") | Some("End") => "match-events",
            Some("Login") | Some("Logout") => "player-sessions",
            _ => "chat-messages",
        }
    } else {
        "chat-messages"
    }
}
