use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    /// Purge all output
    #[arg(short, long)]
    pub purge: bool,
}
