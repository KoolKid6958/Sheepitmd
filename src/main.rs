use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod config;
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
}

fn main() {
    // Inital arg stuff.
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::GenConfig { path }) => {
            config::generate_config(path.to_path_buf());
            println!("Config generated at: {:?}", path);
        }
        None => {}
    }
}
