pub use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add key / value to store
    Add(AddArgs),

    /// Get a value from the store
    Get(GetArgs),

    /// Remove a value from the store
    Rm(RemoveArgs),

    /// Clear the store
    Clear,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    pub key: String,
    pub value: String,
}

#[derive(Args, Debug)]
pub struct GetArgs {
    pub key: Option<String>,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub key: String,
}
