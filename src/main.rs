mod cli;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    if cli.purge {
        utils::mop_and_purge_stdout()?;
    } else {
        utils::mop_stdout()?;
    }

    Ok(())
}
