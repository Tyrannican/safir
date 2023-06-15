mod cli;
mod safir;
mod utils;

use cli::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let store_dir = utils::create_safir_directory()?;
    let mut safir = safir::Safir::init(&store_dir).await?;

    match &cli.command {
        Commands::Add(args) => {
            safir
                .add_entry(args.key.clone(), args.value.clone())
                .await?;
        }
        Commands::Get(args) => {
            if let Some(key) = &args.key {
                safir.get_entry(key.clone())?;
            } else {
                safir.display_all();
            }
        }
        Commands::Rm(args) => {
            safir.remove_entry(args.key.clone()).await?;
        }
        Commands::Alias(args) => {
            safir.set_commands("alias", &args.keys);
        }
        Commands::Export(args) => {
            safir.set_commands("export", &args.keys);
        }
        Commands::Clear => {
            safir.clear_entries().await?;
        }
        Commands::Purge => safir.purge(),
        Commands::Mem(args) => match args {
            MemArgs::Start => println!("Starting mem server!"),
            MemArgs::Stop => println!("Stopping mem server!"),
        },
    }
    Ok(())
}
