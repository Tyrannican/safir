mod cli;
mod store;
mod utils;

use cli::*;

use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut safir = store::init_safir().await.context("loading safir store")?;

    // This is stupid
    match cli.command {
        Commands::Add { key, value } => safir.add(key.to_owned(), value.to_owned()).await?,
        Commands::Get { keys } => safir.get(keys.to_owned()).await?,
        Commands::Rm { keys } => safir.remove(keys.to_owned()).await?,
        Commands::Alias { keys: _ } => {
            // TODO: Fix custom displays
            unimplemented!("alias needs work");
            // safir.custom_display("alias", keys.to_owned()).await?;
        }
        Commands::Export { keys: _ } => {
            unimplemented!("export needs work");
            // safir.custom_display("export", keys.to_owned()).await?;
        }
        Commands::List => safir.list().await?,
        Commands::Clear => safir.clear().await?,
        Commands::Purge => safir.purge().await?,
        Commands::Mode { mode } => {
            let mut cfg = safir.get_config();
            cfg.mode = mode;
            cfg.write().context("writing config out")?;
        }
    }

    // utils::write_store(&safir.store, &safir.file);
    Ok(())
}
