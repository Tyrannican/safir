pub use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Key/Value store to share information between shell sessions", long_about = None)]
pub struct Cli {
    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a value to the store with the given key
    Add(AddArgs),

    /// Get a value from the store
    Get(GetArgs),

    /// Remove a value from the store
    Rm(RemoveArgs),

    /// Clear all keys/values from the store
    Clear,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Name of the item to store
    pub key: String,

    /// Value to store
    pub value: String,
}

#[derive(Args, Debug)]
pub struct GetArgs {
    /// Name of the value to retrieve from the store
    ///
    /// Returns nothing if the key does not exist
    pub key: Option<String>,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Name of the value to remove from the store
    ///
    /// Does nothing if they key does not exist
    pub key: String,
}
