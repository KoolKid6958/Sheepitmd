use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn manage_client(target: &str, instruction: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let data = json!({
        "instruction": instruction,
        "client": target,
    });

    let response = client
        .post("http://localhost:56987")
        .json(&data)
        .send()
        .await?;
    println!("{}", response.text().await?);

    Ok(())
}
