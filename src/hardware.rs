use nvml_wrapper::{Nvml, error::NvmlError};

#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub index: u32,
    pub gpu_name: String,
    pub short_name: String,
}

fn shorten_gpu_name(gpu_name: &str) -> String {
    gpu_name
        .replace("NVIDIA GeForce RTX", "")
        .replace("NVIDIA GeForce GTX", "")
        .replace(" ", "")
        .to_lowercase()
}

pub fn get_nvidia_gpus() -> Result<Vec<GpuInfo>, NvmlError> {
    let nvml = Nvml::init()?;
    let num_gpu = nvml.device_count()?;
    let mut gpus = Vec::new();

    for index in 0..num_gpu {
        let gpu = nvml.device_by_index(index)?;
        let gpu_name = gpu.name()?;
        let short_name = shorten_gpu_name(&gpu_name);
        gpus.push(GpuInfo {
            index,
            gpu_name,
            short_name,
        });
    }
    Ok(gpus)
}
