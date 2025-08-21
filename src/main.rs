use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod config;
mod hardware;

/// CLI Manager for Sheepit.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a default config based on the hardware in the current system.
    #[command(name = "gen-config")]
    GenConfig {
        /// Path to save the config in
        #[arg(
            short,
            long,
            value_name = "FILE",
            default_value = "./.sheepit-manager.toml"
        )]
        path: PathBuf,
    },
    /// List the available GPUs (Nvidia only)
    LsGPU {},
}

fn main() {
    // Inital arg stuff.
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::GenConfig { path }) => {
            config::generate_config(path.to_path_buf());
            println!("Config generated at: {:?}", path);
        }
        Some(Commands::LsGPU {}) => match hardware::get_nvidia_gpus() {
            Ok(_) => {}
            Err(e) => eprintln!(
                "There was an error getting the Nvidia GPUs. Please check that you have Nvidia drivers installed correctly. Err: {}",
                e
            ),
        },
        None => {
            println!("Please run the program with arguments. Use -h to see available options.")
        }
    }
}
