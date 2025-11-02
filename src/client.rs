// This file is meant for managing the sheepit client itself.
use crate::config;
use crate::config::Config;
use reqwest::Client;
use std::path::{Path, PathBuf};
use tokio::{fs, io::AsyncWriteExt};

pub async fn start_client(client: &str) {
    let config_path: PathBuf = "./.sheepit-manager.toml".into();
    let config = config::read_config(config_path.clone());
    println!("Starting: {}", client);
    // CURRENTLY THIS CODE DOESNT USE A SPECIFIC CLIENT. IT ONLY STARTS IT ON CPU WITH DEFAULT SETTINGS.
    let client_path: PathBuf = config.paths.sheepit_client_location.clone();
    match check_if_jar_exists(client_path) {
        true => {}
        false => download_client(config.clone())
            .await
            .expect("There was an error downloading the client"),
    }
}

pub async fn pause_client(_client: &str) {}
pub async fn stop_client(_client: &str) {}
pub async fn stop_client_now(_client: &str) {}
pub async fn client_status(_client: &str) {}

fn check_if_jar_exists(path: PathBuf) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        println!("Client does not exist.");
        false
    }
}

async fn download_client(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading client");
    let url = "https://www.sheepit-renderfarm.com/media/applet/client-latest.php";
    let client_location = config.paths.sheepit_client_location;
    let path = client_location
        .parent()
        .expect("An error occured with the client path. ");

    if !Path::new(&path).exists() {
        println!("Directory {:?} doesnt exist. Creating now.", path);
        fs::create_dir_all(path).await.expect(
            "Failed to create directory. Please check that you have the necessary permissions",
        );
    } else {
    }
    let client = Client::new();
    let mut response = client.get(url).send().await?;
    let mut file = fs::File::create(&client_location).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    println!("Client Downloaded!");
    Ok(())
}
