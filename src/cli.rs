//! CLI for using the Safir binary
pub use clap::{Args, Parser, Subcommand};

/// CLI arguments for running the program
#[derive(Parser, Debug)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Key/Value store to share information between shell sessions", long_about = None)]
pub struct Cli {
    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands for running the program
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a value to the store with the given key
    Add(AddArgs),

    /// Get a value from the store
    Get(GetArgs),

    /// Remove values from the store
    Rm(RemoveArgs),

    /// Output the alias command for a key / value pair to be entered into a shell session
    Alias(SetArgs),

    /// Output the export command for a key / value pair to be entered into a shell session
    Export(SetArgs),

    /// Clear all keys/values from the store
    Clear,

    /// Purges the .safirstore directory, removing it and its contents
    Purge,

    /// Start or stop the Mem service
    #[clap(subcommand)]
    Mem(MemArgs),
}

/// Arguments for adding a value to the store with a given key
#[derive(Args, Debug)]
pub struct AddArgs {
    /// Name of the item to store
    pub key: String,

    /// Value to store
    pub value: String,
}

/// Arguments for retrieving a value from the store with a given key
#[derive(Args, Debug)]
pub struct GetArgs {
    /// Name of the value to retrieve from the store
    ///
    /// Returns nothing if the key does not exist
    pub key: Option<String>,
}

/// Arguments for removing values from the store with given keys
#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Name of the keys to remove from the store
    ///
    /// Does nothing if the keys do not exist
    pub key: Vec<String>,
}

/// Arguments for outputting commands with a given prefix
#[derive(Args, Debug)]
pub struct SetArgs {
    /// Name of the keys to display (e.g. alias / export)
    pub keys: Vec<String>,
}

/// Arguments for the Mem sub command
#[derive(Subcommand, Debug)]
pub enum MemArgs {
    /// Start the Safir Memcache server
    Start,

    /// Stop the Safir Memcache server
    Stop,
}
