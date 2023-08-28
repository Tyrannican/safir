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
use std::io::Result;

use colored::*;

use crate::utils::{confirm_entry, print_header, print_output};
use rubin::net::client::RubinClient;

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
