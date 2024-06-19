mod cli;
mod store;
mod utils;

use cli::*;
use store::Store;

use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut safir = Store::init_safir();

    match &cli.command {
        Commands::Add { key, value } => {
            safir.add(key.to_owned(), value.to_owned());
        }
        Commands::Get(args) => {
            safir.get(args.keys.to_owned());
        }
        Commands::Rm(args) => {
            safir.remove(args.keys.to_owned());
        }
        Commands::Alias(args) => {
            safir.custom_display("alias", args.keys.to_owned());
        }
        Commands::Export(args) => {
            safir.custom_display("export", args.keys.to_owned());
        }
        Commands::List => {
            safir.list();
        }
        Commands::Clear => {
            safir.clear();
        }
        Commands::Purge => {
            safir.purge();
        }
    }

    utils::write_store(&safir.store, &safir.file);
    Ok(())
}
