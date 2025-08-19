use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Serialize)]
struct Config {
    general: General,
    paths: Paths,
    defaults: Defaults,
}

#[derive(Serialize)]
struct General {
    client_name: String,
    shared_zip: bool,
    username: String,
    renderkey: String,
    headless: bool,
}

#[derive(Serialize)]
struct Paths {
    sheepit_cache_dir: PathBuf,
    shared_zip_dir: PathBuf,
    sheepit_client_location: PathBuf,
}

#[derive(Serialize)]
struct Defaults {
    ram: u16,
    cores: u16,
}

pub fn generate_config() {
    let config = Config {
        general: General {
            client_name: "".to_string(),
            shared_zip: false,
            username: "".to_string(),
            renderkey: "".to_string(),
            headless: true,
        },
        paths: Paths {
            sheepit_cache_dir: "/tmp".into(),
            shared_zip_dir: "".into(),
            sheepit_client_location: "/tmp/sheepit-client.jar".into(),
        },
        defaults: Defaults { ram: 0, cores: 0 },
    };
    let toml = toml::to_string(&config).unwrap();
    fs::write(".sheepit-manager.toml", toml).expect("Failed to generate config.");
}
