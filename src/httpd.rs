use axum::{Router, extract::Json, routing::post};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::client;

#[derive(Deserialize)]
struct Instruction {
    instruction: String,
    client: String,
}

pub async fn start() {
    let app = Router::new().route("/", post(get_command));
    let address = SocketAddr::from(([0, 0, 0, 0], 56987));
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Daemon is running on {}", address);
    axum::serve(listener, app).await.unwrap();
}

async fn get_command(Json(payload): Json<Instruction>) -> String {
    match payload.instruction.as_str() {
        "start_client" => client::start_client(&payload.client).await,
        "pause_client" => client::pause_client(&payload.client).await,
        "stop_client" => client::stop_client(&payload.client).await,
        "stop_client_now" => client::stop_client_now(&payload.client).await,
        "get_client_status" => {
            println!("Getting status of: {:?}", payload.client);
            client::client_status(&payload.client).await;
            format!("Status of: {} (This doesnt work yet)", payload.client)
        }
        _ => {
            println!("Unknown");
            println!("{}", &payload.instruction);
            format!("Unknown instruction, are you sure the client and server versions match?")
        }
    }
}
