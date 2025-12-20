use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod client;
mod config;
mod control;
mod hardware;
mod httpd;

/// Daemon for the SheepIt Manager
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true,)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a default config based on the hardware in the current system.
    #[command(name = "gen-config")]
    GenConfig {},
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
        /// Specify what client to stop. If left blank it will stop all running clients.
        #[arg(value_name = "TARGET", default_value = "all")]
        target: String,
    },
    /// Stop clients immediately
    StopNow {
        /// Specify what client to stop. If left blank it will stop all running clients.
        #[arg(value_name = "TARGET", default_value = "all")]
        target: String,
    },
    /// Get the current status of clients
    Status {
        /// Specify what client to get the status of. If left blank it will show the status of all running clients
        #[arg(value_name = "TARGET", default_value = "all")]
        target: String,
    },
    /// Print the config
    PrintConfig {},
    /// Start as the Daemon
    Daemon {},
}

#[tokio::main]
async fn main() {
    // Initial arg stuff.
    let cli = Cli::parse();
    let config_path: PathBuf = "./.sheepit-manager.toml".into();
    match &cli.command {
        Some(Commands::GenConfig {}) => {
            config::generate_config(config_path);
        }
        Some(Commands::LsGPU {}) => match hardware::get_nvidia_gpus() {
            Ok(_) => {}
            Err(e) => eprintln!(
                "There was an error getting the Nvidia GPUs. Please check that you have Nvidia drivers installed correctly. Err: {}",
                e
            ),
        },
        Some(Commands::Start { target }) => {
            let instruction: &str = "start_client";
            let _ = control::manage_client(target, instruction).await;
        }
        Some(Commands::Pause { target }) => {
            let instruction: &str = "pause_client";
            let _ = control::manage_client(target, instruction).await;
        }
        Some(Commands::Stop { target }) => {
            let instruction: &str = "stop_client";
            let _ = control::manage_client(target, instruction).await;
        }
        Some(Commands::StopNow { target }) => {
            let instruction: &str = "stop_client_now";
            let _ = control::manage_client(target, instruction).await;
        }
        Some(Commands::Status { target }) => {
            let instruction: &str = "get_client_status";
            let _ = control::manage_client(target, instruction).await;
        }
        Some(Commands::PrintConfig {}) => config::print_config(config_path),
        Some(Commands::Daemon {}) => {
            httpd::start().await;
        }
        None => {}
    }
}
