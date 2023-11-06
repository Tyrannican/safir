mod cli;
mod utils;
mod store;

use anyhow::Result;
use cli::*;


fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
        }
        Commands::Get(args) => {
        }
        Commands::Rm(args) => {
        }
        Commands::Alias(args) => {
        }
        Commands::Export(args) => {
        }
        Commands::List => {}
        Commands::Clear => {
        }
        Commands::Purge => {
        }
    }

    Ok(())
}
