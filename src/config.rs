use nvml_wrapper::{Nvml, error::NvmlError};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::PathBuf};

#[derive(Serialize)]
struct Config {
    general: General,
    paths: Paths,
    defaults: Defaults,
    cpu: Cpu,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    gpu: BTreeMap<String, Gpu>,
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

#[derive(Serialize)]
struct Cpu {
    ram: u16,
    cores: u16,
}

#[derive(Serialize)]
struct Gpu {
    ram: u16,
    cores: u16,
    optix_id: String,
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
            i = i + 1;
            gpu_map.insert(
                gpu_name,
                Gpu {
                    ram: 0,
                    cores: 0,
                    optix_id: optix_id,
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
            sheepit_cache_dir: "/tmp".into(),
            shared_zip_dir: "".into(),
            sheepit_client_location: "/tmp/sheepit-client.jar".into(),
        },
        defaults: Defaults { ram: 0, cores: 0 },
        cpu: Cpu { ram: 0, cores: 0 },
        gpu: gpu_map,
    };
    let toml = toml::to_string(&config).unwrap();
    fs::write(path, toml).expect("Failed to generate config.");
}
