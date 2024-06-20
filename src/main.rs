mod cli;
mod store;
mod utils;

use cli::*;

use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { key, value } => {
            // safir.add(key.to_owned(), value.to_owned());
        }
        Commands::Get { keys } => {
            // safir.get(args.keys.to_owned());
        }
        Commands::Rm { keys } => {
            // safir.remove(args.keys.to_owned());
        }
        Commands::Alias { keys } => {
            // safir.custom_display("alias", args.keys.to_owned());
        }
        Commands::Export { keys } => {
            // safir.custom_display("export", args.keys.to_owned());
        }
        Commands::List => {
            // safir.list();
        }
        Commands::Clear => {
            // safir.clear();
        }
        Commands::Purge => {
            // safir.purge();
        }
        Commands::Mode { mode } => {
            println!("Mode: {mode:?}");
        }
    }

    // utils::write_store(&safir.store, &safir.file);
    Ok(())
}
