use nvml_wrapper::{Nvml, error::NvmlError};

pub fn get_nvidia_gpus() -> Result<(), NvmlError> {
    let nvml = Nvml::init()?;

    let num_gpu = nvml.device_count()?;
    for i in 0..num_gpu {
        let gpu = nvml.device_by_index(i)?;
        let gpu_name = gpu.name()?;
        println!("GPU Name: {} | Default id: OPTIX_{}", gpu_name, i);
    }
    Ok(())
}
