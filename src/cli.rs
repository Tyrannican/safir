//! CLI for using the Safir binary
pub use clap::{Parser, Subcommand};

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
    Add {
        /// Key for the value
        key: String,

        /// Value to add
        value: String,
    },

    /// Get values from the store
    Get {
        /// Keys for values to retrieve from the store
        keys: Vec<String>,
    },

    /// Remove values from the store
    Rm {
        /// Keys for values to remove from the store
        keys: Vec<String>,
    },

    /// Output the alias command for  key / value pairs
    Alias {
        /// Keys to alias the values
        keys: Vec<String>,
    },

    /// Output the export command for a key / value pairs
    Export {
        /// Keys to export the values
        keys: Vec<String>,
    },

    /// List all values in the store
    List,

    /// Clear all keys/values from the store
    Clear,

    /// Purges the .safirstore directory, removing it and its contents
    Purge,

    /// Use / create an environment to store key / value pairs
    Use {
        /// Name of the environment to use / create
        #[arg(default_value_t = String::from("default"))]
        environment: String,
    },

    /// Display the current loaded environment
    Env,
}
