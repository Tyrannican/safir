mod cli;
mod safir;
use cli::*;
use safir::Safir;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let mut safir = Safir::init()?;

    match &cli.command {
        Commands::Add(args) => {
            safir.add_entry(args.key.clone(), args.value.clone());
        }
        Commands::Get(args) => {
            if let Some(key) = &args.key {
                let entry = safir.get_entry(key.clone());
                println!("{}: {}", key, entry);
            } else {
                safir.display_all();
            }
        }
        Commands::Rm(args) => {
            safir.remove_entry(args.key.clone());
        }
        Commands::Clear => safir.clear_entries(),
    }
    Ok(())
}
