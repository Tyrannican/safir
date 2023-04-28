pub use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add key / value to store
    Add(AddArgs),

    /// Get a value from the store
    Get(GetArgs),

    /// Clear the store
    Clear,
}

#[derive(Args, Debug)]
struct AddArgs {
    key: String,
    value: String,
}

#[derive(Args, Debug)]
struct GetArgs {
    key: String,
}
