// This file is meant for managing the sheepit client itself.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn start_client(client: &str) {
    println!("Starting: {}", client);
    // CURRENTLY THIS CODE DOESNT USE A SPECIFIC CLIENT. IT ONLY STARTS IT ON CPU WITH DEFAULT SETTINGS.
    let path: PathBuf = "/tmp/sheepit/client.jar".into(); // THIS IS TEMP
    match check_if_client_exists(path) {
        true => {}
        false => download_client().expect("There was an error downloading the client"),
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

fn download_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading client");
    let url = "https://www.sheepit-renderfarm.com/media/applet/client-latest.php";
    let path = "/tmp/sheepit/";
    let output = "/tmp/sheepit/client.jar";
    if !Path::new(&output).exists() {
        println!("Directory {} doesnt exist. Creating now.", path);
        fs::create_dir_all(path).expect(
            "Failed to create directory. Please check that you have the necessary permissions",
        );
    } else {
    }
    let status = Command::new("curl")
        .args(["-#", "-L", "-o", output, url])
        .status()?;
    if !status.success() {
        return Err(format!("curl failed with exit code {:?}", status.code()).into());
    }
    println!("Client Downloaded!");
    Ok(())
}
