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
use anyhow::Result;

use async_trait::async_trait;
use colored::*;

use crate::config::SafirConfig;
use crate::utils::{confirm_entry, print_header, print_headless, print_output};
use crate::SafirEngine;
use rubin::store::persistence::PersistentStore;

/// Safir Store (fancy wrapper around reading and writing to a JSON file)
pub struct SafirStore {
    pub store: PersistentStore,
    pub headless: bool,
}

impl SafirStore {
    /// Initialises the Safirstore if not already initialised
    pub async fn new(config: &SafirConfig) -> Result<Self> {
        let store_path = config.root_path.join("safirstore.json");
        let mut ps = if store_path.exists() {
            PersistentStore::from_existing(store_path).await?
        } else {
            PersistentStore::new(store_path).await?
        };

        ps.set_write_on_update(true);
        let headless = config.get_headless_mode();

        Ok(Self {
            store: ps,
            headless,
        })
    }

    /// Display all key/values in the store
    pub fn display_all(&self) {
        let strings = self.store.get_string_store_ref();
        if self.headless {
            for (key, value) in strings.iter() {
                print_headless("", key, value);
            }

            return;
        }

        print_header();
        let mut output: String;
        for (key, value) in strings.iter() {
            output = format!("{}: \"{}\"", key.bold().yellow(), value);
            print_output(&output);
        }
    }

    /// Remove the store directory and all contents
    pub fn purge(&self) {
        if confirm_entry("Are you sure you want to purge Safirstore?") {
            std::fs::remove_dir_all(&self.store.path)
                .expect("unable to remove safirstore directory");
        }
    }
}

#[async_trait]
impl SafirEngine for SafirStore {
    /// Add an entry to the store and write it out to disk
    async fn add_entry(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert_string(&key, &value).await?;
        Ok(())
    }

    /// Get an entry form the store by loading it from disk and displaying it
    async fn get_entry(&self, key: String) -> Result<()> {
        let value = if let Ok(val) = self.store.get_string(&key) {
            val
        } else {
            String::from("")
        };

        if self.headless {
            print_headless("", &key, &value);
            return Ok(());
        }

        print_header();
        let output = if !value.is_empty() {
            format!("{}: \"{}\"", key.bold().yellow(), value)
        } else {
            format!("{}: ", key.bold().yellow())
        };

        print_output(&output);

        Ok(())
    }

    /// Remove a key/value pair from the store and update onto disk
    async fn remove_entry(&mut self, keys: Vec<String>) -> Result<()> {
        for key in &keys {
            self.store.remove_string(key).await?;
        }

        Ok(())
    }

    /// Outputs the key/value pair as a command with the prefix
    ///
    /// E.g. With a prefix of `alias` this will display the command as
    /// `alias {key}="{value}"` with {key} / {value} replaced with their values from the store
    async fn set_commands(&mut self, prefix: &str, keys: &Vec<String>) {
        if self.headless {
            for key in keys {
                if let Ok(value) = self.store.get_string(key) {
                    print_headless(prefix, key, &value);
                }
            }

            return;
        }

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
    async fn clear_entries(&mut self) -> Result<()> {
        if confirm_entry("Are you sure you want to clear the store?") {
            self.store.clear_strings().await?;
        }

        Ok(())
    }

    fn to_type(&self) -> &dyn std::any::Any {
        self
    }
}
