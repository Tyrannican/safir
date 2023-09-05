use anyhow::Result;

use async_trait::async_trait;

use crate::config::SafirConfig;
use crate::utils::{self, confirm_entry, print_headless};
use crate::SafirEngine;
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
    pub async fn new(cfg: &SafirConfig) -> Result<Self> {
        Ok(Self {
            is_online: utils::is_safir_running(cfg.memcache_pid),
            client: RubinClient::new("127.0.0.1", 9876),
        })
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

#[async_trait]
impl SafirEngine for SafirMemcache {
    async fn add_entry(&mut self, key: String, value: String) -> Result<()> {
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

        self.client.insert_string(&key, &value).await?;
        Ok(())
    }

    async fn get_entry(&self, key: String) -> Result<()> {
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

        let value = if let Ok(val) = self.client.get_string(&key).await {
            val
        } else {
            String::from("")
        };

        print_headless("", &key, &value);

        Ok(())
    }

    async fn remove_entry(&mut self, keys: Vec<String>) -> Result<()> {
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

        for key in &keys {
            self.client.remove_string(key).await?;
        }

        Ok(())
    }

    async fn set_commands(&mut self, prefix: &str, keys: &Vec<String>) {
        if !self.is_online {
            safir_offline();
            return;
        }

        for key in keys {
            if let Ok(value) = self.client.get_string(key).await {
                print_headless(prefix, key, &value);
            }
        }
    }

    async fn clear_entries(&mut self) -> Result<()> {
        if !self.is_online {
            safir_offline();
            return Ok(());
        }

        if confirm_entry("Are you sure you want to clear the store?") {
            self.client.clear_strings().await?;
        }

        Ok(())
    }

    fn to_type(&self) -> &dyn std::any::Any {
        self
    }
}
