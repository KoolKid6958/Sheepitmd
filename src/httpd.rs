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
        "start_client" => {
            println!("Starting client: {:?}", payload.client);
            client::start_client(&payload.client).await;
            format!("Starting client: {:?}", payload.client)
        }
        "pause_client" => {
            println!("Pausing client: {:?}", payload.client);
            client::pause_client(&payload.client).await;
            format!("Pausing client: {:?}", payload.client)
        }
        "stop_client" => {
            println!("Stopping client: {:?}", payload.client);
            client::stop_client(&payload.client).await;
            format!("Stopping client: {:?}", payload.client)
        }
        "stop_client_now" => {
            println!("Stopping client: {:?} Now", payload.client);
            client::stop_client_now(&payload.client).await;
            format!("Stopping client: {:?} Now", payload.client)
        }
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
