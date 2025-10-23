use axum::{Router, extract::Json, routing::post};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Message {
    message: String,
}

pub async fn start() {
    let app = Router::new().route("/", post(get_command));
    let address = SocketAddr::from(([0, 0, 0, 0], 56987));
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Daemon is running on {}", address);
    axum::serve(listener, app).await.unwrap();
}

async fn get_command(Json(payload): Json<Message>) -> &'static str {
    match payload.message.as_str() {
        "Hello" => {
            println!("World!");
            "Got Message."
        }
        _ => {
            println!("Unknown");
            "Unknown"
        }
    }
}
