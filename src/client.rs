// This file is meant for managing the sheepit client itself.
use crate::config;
use crate::config::Config;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;
use tokio::{fs, io::AsyncWriteExt, process::Child, process::Command};

static CLIENTS: Lazy<Mutex<HashMap<String, Child>>> = Lazy::new(|| Mutex::new(HashMap::new()));

async fn send_command_to_process(client_id: &str, command: &str) -> String {
    let mut map = CLIENTS.lock().await;

    if let Some(child) = map.get_mut(client_id) {
        if let Some(stdin) = child.stdin.as_mut() {
            if let Err(e) = stdin.write_all(command.as_bytes()).await {
                eprintln!(
                    "Failed to send command '{}' to client '{}': {}",
                    command.trim_end(),
                    client_id,
                    e
                );
                return format!(
                    "Failed to send command '{}' to client '{}': {}",
                    command.trim_end(),
                    client_id,
                    e,
                );
            }
            if let Err(e) = stdin.flush().await {
                eprintln!(
                    "Failed to flush command '{}' to client '{}': {}",
                    command.trim_end(),
                    client_id,
                    e
                );
                return format!(
                    "Failed to flush command '{}' to client '{}': {}",
                    command.trim_end(),
                    client_id,
                    e
                );
            } else {
                println!(
                    "Sent command '{}' to client '{}'",
                    command.trim_end(),
                    client_id
                );
                return format!(
                    "Sent command '{}' to client '{}'",
                    command.trim_end(),
                    client_id
                );
            }
        } else {
            eprintln!("Client '{}' has no stdin handle", client_id);
            return format!("Client '{}' has no stdin handle", client_id);
        }
    } else {
        println!("Client '{}' is not running.", client_id);
        return format!("Client '{}' is not running.", client_id);
    }
}

pub async fn start_client(client: &str) -> String {
    let config_path: PathBuf = "./.sheepit-manager.toml".into();
    let config = config::read_config(config_path.clone());
    {
        let map = CLIENTS.lock().await;
        if map.contains_key(client) {
            println!("Cannot start {:?} because it is already running.", client);
            return format!("Cannot start {:?} because it's already running.", client);
        }
    }
    println!("Starting: {}", client);
    // CURRENTLY THIS CODE DOESNT USE A SPECIFIC CLIENT. IT ONLY STARTS IT ON CPU WITH DEFAULT SETTINGS.
    let client_path: PathBuf = config.paths.sheepit_client_location.clone();
    match check_if_jar_exists(client_path.clone()) {
        true => {}
        false => download_client(config.clone())
            .await
            .expect("There was an error downloading the client"),
    }
    // Checks if the log path exists, it seems sheepit wont auto create it so we have to.
    let mut log_path: PathBuf = config.paths.log_dir.clone();
    log_path.push(client);

    if !Path::new(&log_path).exists() {
        fs::create_dir_all(log_path).await.expect(
            "Failed to create directory. Please check that you have the necessary permissions",
        );
    } else {
    }
    // Set config options for the client
    let child = Command::new("java")
        .arg("-jar")
        .arg(&client_path)
        .arg("-ui")
        .arg("text")
        .arg("-login")
        .arg(config.general.username)
        .arg("-password")
        .arg(config.general.renderkey)
        .arg("-cores")
        .arg(config.defaults.cores.to_string())
        .arg("-memory")
        .arg(format!("{}G", config.defaults.ram))
        .arg("-server")
        .arg(config.general.server)
        .arg("-cache-dir")
        .arg(format!(
            "{}/{}",
            config.paths.sheepit_cache_dir.display(),
            &client
        ))
        .arg("-hostname")
        .arg(format!("{}-{}", config.general.client_name, client))
        .arg("-logdir")
        .arg(format!("{}/{}", config.paths.log_dir.display(), client))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn client");
    {
        let mut map = CLIENTS.lock().await;
        map.insert(client.to_string(), child);
        format!("Client {:?} started.", client)
    }
}

pub async fn pause_client(client: &str) -> String {
    send_command_to_process(client, "pause\n").await
}
pub async fn stop_client(client: &str) -> String {
    send_command_to_process(client, "stop\n").await
}
pub async fn stop_client_now(client: &str) -> String {
    send_command_to_process(client, "quit\n").await
}
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
        .expect("An error occurred with the client path. ");

    if !Path::new(&path).exists() {
        println!("Directory {:?} doesn't exist. Creating now.", path);
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
