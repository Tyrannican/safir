mod cli;
mod store;
mod utils;

use cli::*;

use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut safir = store::init_safir().await.context("loading safir store")?;

    match cli.command {
        Commands::Add { key, value } => safir.add(key.to_owned(), value.to_owned()).await?,
        Commands::Get { keys } => {
            let kvs = safir.get(keys.to_owned()).await?;
            utils::display_multiple_kv(kvs);
        }
        Commands::Rm { keys } => safir.remove(keys.to_owned()).await?,
        Commands::Alias { keys } => {
            let kvs = safir.get(keys.to_owned()).await?;
            utils::custom_display("alias", kvs);
        }
        Commands::Export { keys } => {
            let kvs = safir.get(keys.to_owned()).await?;
            utils::custom_display("export", kvs);
        }
        Commands::List => {
            let kvs = safir.list().await?;
            utils::display_multiple_kv(kvs);
        }
        Commands::Clear => safir.clear().await?,
        Commands::Purge => safir.purge().await?,
        Commands::Mode { mode } => {
            let mut cfg = safir.get_config();
            cfg.mode = mode;
            cfg.write().context("writing config out")?;
        }
    }

    Ok(())
}
