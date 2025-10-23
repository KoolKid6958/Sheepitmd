use clap::{Parser, Subcommand};
use std::{io, io::Write, path::PathBuf};

pub mod client;
mod config;
mod hardware;
mod httpd;

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
    /// Print the config
    PrintConfig {},
    /// Start as the Daemon
    Daemon {},
}

#[tokio::main]
async fn main() {
    // Inital arg stuff.
    let cli = Cli::parse();
    let config_path: PathBuf = "./.sheepit-manager.toml".into();
    match &cli.command {
        Some(Commands::GenConfig {}) => {
            // Check if the file exists
            if config_path.exists() {
                print!("The file exists. Would you like to overwrite it? (y/N): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin()
                    .read_line(&mut confirm)
                    .expect("There was an error");
                let confirm = confirm.trim().to_lowercase();
                if confirm == "y" {
                    config::generate_config(config_path.to_path_buf());
                    println!("Config generated at: {:?}", config_path);
                } else {
                    println!("Exiting")
                }
            } else {
                config::generate_config(config_path.to_path_buf());
                println!("Config generated at: {:?}", config_path);
            }
        }
        Some(Commands::LsGPU {}) => match hardware::get_nvidia_gpus() {
            Ok(_) => {}
            Err(e) => eprintln!(
                "There was an error getting the Nvidia GPUs. Please check that you have Nvidia drivers installed correctly. Err: {}",
                e
            ),
        },
        Some(Commands::Start { target }) => {
            client::start_client(&target, config_path);
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
        Some(Commands::PrintConfig {}) => config::print_config(config_path),
        None => {
            println!("Please run the program with arguments. Use -h to see available options.")
        }
        Some(Commands::Daemon {}) => {
            httpd::start().await;
        }
    }
}
