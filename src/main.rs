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
    /// Start clients
    Start {
        /// Specify what client to start. If left blank it will start all clients that are enabled in the config file.
        #[arg(value_name = "TARGET", default_value = "all")]
        target: String,
    },
    /// Pause clients
    Pause {
        /// Specify what client to pause. If left blank it will pause all running clients.
        #[arg(value_name = "TARGET", default_value = "all")]
        target: String,
    },
    /// Stop clients
    Stop {
        /// Specify what client to stop. If left blank it will stop all running clients. Add --now to stop them immediately.
        #[arg(value_name = "TARGET", default_value = "all")]
        target: String,
        #[arg(long)]
        now: bool,
    },
    /// Get the current status of clients
    Status {
        /// Specify what client to get the status of. If left blank it will show the status of all running clients
        #[arg(value_name = "TARGET", default_value = "all")]
        target: String,
    },
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
        Some(Commands::Start { target }) => {
            println!("{}", target);
        }
        Some(Commands::Pause { target }) => {
            println!("{}", target);
        }
        Some(Commands::Stop {
            target,
            now: stop_now,
        }) => {
            println!("{}", target);
            if *stop_now {
                println!("stop now")
            }
        }
        Some(Commands::Status { target }) => {
            println!("{}", target);
        }
        None => {
            println!("Please run the program with arguments. Use -h to see available options.")
        }
    }
}
