use nvml_wrapper::{Nvml, error::NvmlError};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    general: General,
    paths: Paths,
    defaults: Defaults,
    cpu: Cpu,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    gpu: BTreeMap<String, Gpu>,
}

#[derive(Serialize, Deserialize)]
struct General {
    client_name: String,
    shared_zip: bool,
    username: String,
    renderkey: String,
    headless: bool,
}

#[derive(Serialize, Deserialize)]
struct Paths {
    sheepit_cache_dir: PathBuf,
    shared_zip_dir: PathBuf,
    sheepit_client_location: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Defaults {
    ram: u16,
    cores: u16,
}

#[derive(Serialize, Deserialize)]
struct Cpu {
    ram: u16,
    cores: u16,
    enabled: bool,
}

#[derive(Serialize, Deserialize)]
struct Gpu {
    ram: u16,
    cores: u16,
    optix_id: String,
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

pub fn generate_config(path: PathBuf) {
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
        },
        paths: Paths {
            sheepit_cache_dir: "/tmp/sheepit/cache".into(),
            shared_zip_dir: "".into(),
            sheepit_client_location: "/tmp/sheepit/client.jar".into(),
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
    fs::write(path, toml).expect("Failed to generate config.");
}

pub fn read_config(path: PathBuf) -> Config {
    let raw_file =
        fs::read_to_string(path).expect("Failed to read config file, have you generated one yet?");
    let config: Config = toml::from_str(&raw_file)
        .expect("Failed to parse Toml. Please ensure your config is valid.");
    config
}

pub fn print_config(path: PathBuf) {
    let config = read_config(path);
    println!(
        "User: {}, Client Name: {}",
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
