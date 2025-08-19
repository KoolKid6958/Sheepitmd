use clap::Parser;
/// CLI Manager for Sheepit.
#[derive(Parser)]
#[command(version, about, long_about = None)]

struct Args {
    /// Generate a default config based on the hardware in the current system.
    #[arg(long)]
    genconfig: bool,
}
fn config() {
    println!("Config");
}
fn main() {
    //Inital arg stuff
    let args = Args::parse();
    if args.genconfig {
        config();
    }
}
