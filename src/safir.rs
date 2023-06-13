//! Safir handles the logic of writing key/value pairs to disk
//!
//! Provides a simple wrapper around a Hash Map which stores values  with given keys
//! to a JSON file on disk so that you can load them up in other shell sessions.
//!
//! Prevents the faff around having to add rely on shell history or RC files for items
//! that you don't usually want to persist around.
//!
//! Safir gives you the option to add / get / remove items from the store
//! and to clear / purge when you're finished with them.
use std::io::{Result, Write};

use colored::*;

use rubin::store::persistence::PersistentStore;

/// Formats and prints the message to stdout
fn print_output(msg: &str) {
    println!("{}", format!("{}\n", msg));
}

/// Prints the Safirstore header
fn print_header() {
    println!("{}", "--=Safirstore=--\n".bold());
}

/// Safir Store (fancy wrapper around reading and writing to a JSON file)
pub struct Safir {
    pub store: PersistentStore,
}

impl Safir {
    /// Initialises the Safirstore if not already initialised
    pub async fn init() -> Result<Self> {
        let home_dir = dirs::home_dir().unwrap();
        let store_path = home_dir.join(".safirstore/safirstore.json");
        let mut ps = if store_path.exists() {
            PersistentStore::from_existing(store_path).await?
        } else {
            PersistentStore::new(store_path).await?
        };

        ps.set_write_on_update(true);
        Ok(Self { store: ps })
    }

    /// Add an entry to the store and write it out to disk
    pub async fn add_entry(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert_string(&key, &value).await
    }

    /// Get an entry form the store by loading it from disk and displaying it
    pub fn get_entry(&self, key: String) -> Result<()> {
        print_header();
        let output = if let Ok(val) = self.store.get_string(&key) {
            format!("{}: \"{}\"", key.bold().yellow(), val)
        } else {
            format!("{}: ", key.bold().yellow())
        };

        print_output(&output);

        Ok(())
    }

    /// Display all key/values in the store
    pub fn display_all(&self) {
        print_header();
        let mut output: String;
        let strings = self.store.get_string_store_ref();
        for (key, value) in strings.iter() {
            output = format!("{}: \"{}\"", key.bold().yellow(), value);
            print_output(&output);
        }
    }

    /// Remove a key/value pair from the store and update onto disk
    pub async fn remove_entry(&mut self, keys: Vec<String>) -> Result<()> {
        for key in &keys {
            self.store.remove_string(key).await?;
        }

        Ok(())
    }

    /// Outputs the key/value pair as a command with the prefix
    ///
    /// E.g. With a prefix of `alias` this will display the command as
    /// `alias {key}="{value}"` with {key} / {value} replaced with their values from the store
    pub fn set_commands(&self, prefix: &str, keys: &Vec<String>) {
        print_header();
        let prefix = match prefix {
            "alias" => "alias".bold().green(),
            "export" => "export".bold().magenta(),
            _ => prefix.bold(),
        };

        for key in keys {
            if let Ok(value) = self.store.get_string(key) {
                println!("{} {}=\"{}\"\n", prefix, key.bold().yellow(), value);
            }
        }
    }

    /// Clear the the contents of the store and update onto disk
    pub async fn clear_entries(&mut self) -> Result<()> {
        if self.confirm_entry("Are you sure you want to clear the store?") {
            self.store.clear_strings().await?;
        }

        Ok(())
    }

    /// Remove the store directory and all contents
    pub fn purge(&mut self) {
        if self.confirm_entry("Are you sure you want to purge Safirstore?") {
            std::fs::remove_dir_all(&self.store.path)
                .expect("unable to remove safirstore directory");
        }
    }

    /// Confirmation dialog for important calls
    fn confirm_entry(&self, msg: &str) -> bool {
        let mut answer = String::new();
        print!("{} (y/n) ", msg);
        std::io::stdout().flush().expect("failed to flush buffer");

        let _ = std::io::stdin()
            .read_line(&mut answer)
            .expect("unable to get input from user");

        let answer = answer.trim();
        if answer == "y" || answer == "Y" {
            return true;
        }

        false
    }
}
