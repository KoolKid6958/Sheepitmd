use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io, io::Write, path::PathBuf};

use crate::hardware;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: General,
    pub paths: Paths,
    pub defaults: Defaults,
    pub cpu: Cpu,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub gpu: BTreeMap<String, Gpu>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct General {
    pub client_name: String,
    pub shared_zip: bool,
    pub username: String,
    pub renderkey: String,
    pub headless: bool,
    pub server: String,
    pub debug: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Paths {
    pub sheepit_cache_dir: PathBuf,
    pub shared_zip_dir: PathBuf,
    pub sheepit_client_location: PathBuf,
    pub log_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Defaults {
    pub ram: u16,
    pub cores: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cpu {
    pub ram: u16,
    pub cores: u16,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gpu {
    pub ram: u16,
    pub cores: u16,
    pub optix_id: String,
    enabled: bool,
}

fn check_if_config_exists(config_path: &PathBuf) -> bool {
    // Check if the config file exists, if it does, confirm with the user before overwriting.
    if config_path.exists() {
        print!("The file exists. Would you like to overwrite it? (y/N): ");
        io::stdout().flush().unwrap();

        let mut confirm = String::new();
        io::stdin()
            .read_line(&mut confirm)
            .expect("There was an error");
        let confirm = confirm.trim().to_lowercase();
        if confirm == "y" { true } else { false }
    } else {
        true
    }
}

pub fn generate_config(config_path: PathBuf) {
    if check_if_config_exists(&config_path) {
        // Dynamic GPU Map
        let mut gpu_map = BTreeMap::new();
        if let Ok(gpus) = hardware::get_nvidia_gpus() {
            for i in gpus {
                let optix_id = format!("OPTIX_{}", i.index);
                let key = format!("{}-{}", i.short_name, i.index);
                gpu_map.insert(
                    key,
                    Gpu {
                        ram: 0,
                        cores: 0,
                        optix_id,
                        enabled: true,
                    },
                );
            }
        }
        // Write to the file
        let config = Config {
            general: General {
                client_name: "".to_string(),
                shared_zip: false,
                username: "".to_string(),
                renderkey: "".to_string(),
                headless: true,
                server: "https://sheepit-renderfarm.com".to_string(),
                debug: false,
            },
            paths: Paths {
                sheepit_cache_dir: "/tmp/sheepitm/cache".into(),
                shared_zip_dir: "".into(),
                sheepit_client_location: "/tmp/sheepitm/client.jar".into(),
                log_dir: "/tmp/sheepitm/logs".into(),
            },
            defaults: Defaults { ram: 0, cores: 0 },
            cpu: Cpu {
                ram: 0,
                cores: 0,
                enabled: false,
            },
            gpu: gpu_map,
        };
        let toml = toml::to_string(&config).unwrap();
        fs::write(&config_path, toml).expect("Failed to generate config.");
        println!("Config generated at: {:?}", config_path);
    }
}

pub fn read_config(config_path: PathBuf) -> Config {
    let raw_file = fs::read_to_string(config_path)
        .expect("Failed to read config file, have you generated one yet?");
    let config: Config = toml::from_str(&raw_file)
        .expect("Failed to parse Toml. Please ensure your config is valid.");
    config
}

pub fn print_config(config_path: PathBuf) {
    let config = read_config(config_path.clone());
    println!(
        "User: {:?}, Client Name: {:?}",
        config.general.username, config.general.client_name
    );
    println!(
        "Using Cache dir: {:?} and client: {:?}",
        config.paths.sheepit_cache_dir, config.paths.sheepit_client_location
    );
    println!(
        "CPU -> Cores: {}, RAM: {}, Enabled: {}",
        config.cpu.cores, config.cpu.ram, config.cpu.enabled
    );
    for (key, gpu) in &config.gpu {
        println!(
            "{} -> Cores: {}, RAM: {}, Optix ID: {}, Enabled: {}",
            key, gpu.cores, gpu.ram, gpu.optix_id, gpu.enabled
        );
    }
}
