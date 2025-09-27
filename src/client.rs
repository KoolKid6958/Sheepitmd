// This file is meant for managing the sheepit client itself.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::config::Config;

pub fn start_client(client: &str, config: Config) {
    println!("Starting: {}", client);
    // CURRENTLY THIS CODE DOESNT USE A SPECIFIC CLIENT. IT ONLY STARTS IT ON CPU WITH DEFAULT SETTINGS.
    let client_path: PathBuf = config.paths.sheepit_client_location.clone();
    match check_if_client_exists(client_path) {
        true => {}
        false => download_client(config).expect("There was an error downloading the client"),
    }
}

fn check_if_client_exists(path: PathBuf) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        println!("Client does not exist.");
        false
    }
}

fn download_client(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading client");
    let url = "https://www.sheepit-renderfarm.com/media/applet/client-latest.php";
    let client_location = config.paths.sheepit_client_location;
    let path = client_location
        .parent()
        .expect("An error occured with the client path. ");

    if !Path::new(&path).exists() {
        println!("Directory {:?} doesnt exist. Creating now.", path);
        fs::create_dir_all(path).expect(
            "Failed to create directory. Please check that you have the necessary permissions",
        );
    } else {
    }
    let status = Command::new("curl")
        .args(["-#", "-L", "-o", client_location.to_str().unwrap(), url])
        .status()?;
    if !status.success() {
        return Err(format!("curl failed with exit code {:?}", status.code()).into());
    }
    println!("Client Downloaded!");
    Ok(())
}
