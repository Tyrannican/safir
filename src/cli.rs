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
    Add { key: String, value: String },

    /// Get values from the store
    Get(GetArgs),

    /// Remove values from the store
    Rm(RemoveArgs),

    /// Output the alias command for  key / value pairs
    Alias(SetArgs),

    /// Output the export command for a key / value pairs
    Export(SetArgs),

    /// List all values in the store
    List,

    /// Clear all keys/values from the store
    Clear,

    /// Purges the .safirstore directory, removing it and its contents
    Purge,
}

/// Arguments for retrieving values from the store with the given keys
#[derive(Args, Debug)]
pub struct GetArgs {
    /// Keys to retrieve the values for
    ///
    /// Returns nothing if the key does not exist
    pub keys: Vec<String>,
}

/// Arguments for removing values from the store with given keys
#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Name of the keys to remove from the store
    ///
    /// Does nothing if the keys do not exist
    pub keys: Vec<String>,
}

/// Arguments for outputting commands with a given prefix
#[derive(Args, Debug)]
pub struct SetArgs {
    /// Name of the keys to display (e.g. alias / export)
    pub keys: Vec<String>,
}
