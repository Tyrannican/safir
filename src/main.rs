mod cli;
mod safir;
use cli::*;
use safir::Safir;

fn main() -> std::io::Result<()> {
    // let cli = Cli::parse();
    let mut safir = Safir::init()?;
    Ok(())
}
