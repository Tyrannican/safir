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
use std::{io::Result, path::Path};

use colored::*;

use crate::utils::{confirm_entry, print_header, print_output};
use rubin::{net::client::RubinClient, store::persistence::PersistentStore};

/// Safir Store (fancy wrapper around reading and writing to a JSON file)
pub struct Safir {
    pub store: PersistentStore,
}

impl Safir {
    /// Initialises the Safirstore if not already initialised
    pub async fn init(store_loc: &Path) -> Result<Self> {
        let store_path = store_loc.join("safirstore.json");
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
        if confirm_entry("Are you sure you want to clear the store?") {
            self.store.clear_strings().await?;
        }

        Ok(())
    }

    /// Remove the store directory and all contents
    pub fn purge(&mut self) {
        if confirm_entry("Are you sure you want to purge Safirstore?") {
            std::fs::remove_dir_all(&self.store.path)
                .expect("unable to remove safirstore directory");
        }
    }
}

pub struct SafirMemcache {
    client: RubinClient,
}

impl SafirMemcache {
    pub fn new() -> Self {
        Self {
            client: RubinClient::new("127.0.0.1", 9876),
        }
    }

    pub async fn add_entry(&self, key: &str, value: &str) -> Result<()> {
        self.client.insert_string(key, value).await?;
        Ok(())
    }

    pub async fn get_string(&self, key: &str) -> Result<()> {
        print_header();
        let output = if let Ok(val) = self.client.get_string(key).await {
            format!("{}: \"{}\"", key.bold().yellow(), val)
        } else {
            format!("{}: ", key.bold().yellow())
        };

        print_output(&output);

        Ok(())
    }

    pub async fn remove_entry(&self, keys: Vec<String>) -> Result<()> {
        for key in &keys {
            self.client.remove_string(key).await?;
        }

        Ok(())
    }

    pub async fn set_commands(&self, prefix: &str, keys: &Vec<String>) {
        print_header();
        let prefix = match prefix {
            "alias" => "alias".bold().green(),
            "export" => "export".bold().magenta(),
            _ => prefix.bold(),
        };

        for key in keys {
            if let Ok(value) = self.client.get_string(key).await {
                println!("{} {}=\"{}\"\n", prefix, key.bold().yellow(), value);
            }
        }
    }

    pub async fn clear_entries(&self) -> Result<()> {
        if confirm_entry("Are you sure you want to clear the store?") {
            self.client.clear_strings().await?;
        }

        Ok(())
    }

    pub async fn dump_store(&self, path: &str) -> Result<()> {
        self.client.dump_store(path).await?;
        println!("Safir memcache dumped to {}", path);

        Ok(())
    }
}
