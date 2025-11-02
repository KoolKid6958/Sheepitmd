use nvml_wrapper::{Nvml, error::NvmlError};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};

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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Paths {
    pub sheepit_cache_dir: PathBuf,
    pub shared_zip_dir: PathBuf,
    pub sheepit_client_location: PathBuf,
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
fn shorten_gpu_name(gpu_name: &str) -> String {
    gpu_name
        .replace("NVIDIA GeForce RTX", "")
        .replace("NVIDIA GeForce GTX", "")
        .replace(" ", "")
        .to_lowercase()
}

fn get_gpus() -> Result<Vec<String>, NvmlError> {
    let nvml = Nvml::init()?;
    let num_gpu = nvml.device_count()?;
    let mut gpus = Vec::new();

    for i in 0..num_gpu {
        let gpu = nvml.device_by_index(i)?;
        let gpu_name = gpu.name()?;
        let short_name = shorten_gpu_name(&gpu_name);
        gpus.push(short_name);
    }
    Ok(gpus)
}

pub fn generate_config(config_path: PathBuf) {
    // Dynamic GPU Map
    let mut gpu_map = BTreeMap::new();
    if let Ok(gpus) = get_gpus() {
        let mut i = 0;
        for gpu_name in gpus {
            let optix_id = format!("OPTIX_{}", i);
            let key = format!("{}-{}", gpu_name, i);
            i = i + 1;
            gpu_map.insert(
                key,
                Gpu {
                    ram: 0,
                    cores: 0,
                    optix_id: optix_id,
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
        },
        paths: Paths {
            sheepit_cache_dir: "/tmp/sheepitm/cache".into(),
            shared_zip_dir: "".into(),
            sheepit_client_location: "/tmp/sheepitm/client.jar".into(),
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
    fs::write(config_path, toml).expect("Failed to generate config.");
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
