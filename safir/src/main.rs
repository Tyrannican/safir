mod cli;

use anyhow::Result;
use cli::*;

use safir_core::{Safir, SafirEngineType};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut safir = Safir::new(SafirEngineType::Store).await?;

    match &cli.command {
        Commands::Add(args) => {
            safir
                .add_entry(args.key.clone(), args.value.clone())
                .await?;
        }
        Commands::Get(args) => {
            if let Some(key) = &args.key {
                safir.get_entry(key.clone()).await?;
            } else {
                let inner = safir.as_safir_store();
                inner.display_all();
            }
        }
        Commands::Rm(args) => {
            safir.remove_entry(args.key.clone()).await?;
        }
        Commands::Alias(args) => {
            safir.set_commands("alias", &args.keys).await;
        }
        Commands::Export(args) => {
            safir.set_commands("export", &args.keys).await;
        }
        Commands::Clear => {
            safir.clear_entries().await?;
        }
        Commands::Purge => {
            let inner = safir.as_safir_store();
            inner.purge();
        }
        Commands::Headless(mode) => match mode {
            HeadlessFlags::On => {
                safir.config.headless_mode = Some(true);
                safir.config.write().await?;
                println!("Headless mode is ON");
            }
            HeadlessFlags::Off => {
                safir.config.headless_mode = Some(false);
                safir.config.write().await?;
                println!("Headless mode is OFF");
            }
        },
    }

    Ok(())
}
