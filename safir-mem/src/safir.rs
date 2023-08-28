use std::io::Result;

use colored::*;

use crate::utils::{confirm_entry, print_header, print_output};
use rubin::net::client::RubinClient;

fn safir_offline() {
    eprintln!(
        "Safir Memcache does not appear to be online.\nStart it by running `safir-mem start`."
    );
}

pub struct SafirMemcache {
    is_online: bool,
    client: RubinClient,
}

impl SafirMemcache {
    pub fn new(is_online: bool) -> Self {
        Self {
            is_online,
            client: RubinClient::new("127.0.0.1", 9876),
        }
    }

    pub async fn add_entry(&self, key: &str, value: &str) -> Result<()> {
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

        self.client.insert_string(key, value).await?;
        Ok(())
    }

    pub async fn get_string(&self, key: &str) -> Result<()> {
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

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
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

        for key in &keys {
            self.client.remove_string(key).await?;
        }

        Ok(())
    }

    pub async fn set_commands(&self, prefix: &str, keys: &Vec<String>) {
        if !self.is_online {
            safir_offline();
            return;
        }

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
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

        if confirm_entry("Are you sure you want to clear the store?") {
            self.client.clear_strings().await?;
        }

        Ok(())
    }

    pub async fn dump_store(&self, path: &str) -> Result<()> {
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

        self.client.dump_store(path).await?;
        println!("Safir memcache dumped to {}", path);

        Ok(())
    }
}
