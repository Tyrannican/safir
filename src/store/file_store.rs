use crate::{
    store::{config::SafirConfig, SafirStore},
    utils::{self, KVPair},
};

use anyhow::Result;
use async_trait::async_trait;

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone)]
pub struct KVStore {
    loc: PathBuf,
    store: HashMap<String, String>,
    config: SafirConfig,
}

impl KVStore {
    pub fn load(ws: PathBuf, config: SafirConfig) -> Self {
        let store_path = ws.join("safirstore.json");
        let store = if store_path.exists() {
            utils::load_store(&store_path)
        } else {
            let store = HashMap::new();
            utils::write_store(&store, &store_path);
            store
        };

        Self {
            loc: store_path,
            config,
            store,
        }
    }
}

#[async_trait]
impl SafirStore for KVStore {
    async fn add(&mut self, key: String, value: String) -> Result<()> {
        if let Some(v) = self.store.get(&key) {
            let confirm_msg = format!("Key {key} already exists ({v}), Replace?");
            if utils::confirm_entry(&confirm_msg) {
                self.store.insert(key, value);
            }
        } else {
            self.store.insert(key, value);
        }

        utils::write_store(&self.store, &self.loc);

        Ok(())
    }

    async fn get(&self, keys: Vec<String>) -> Result<Vec<KVPair>> {
        let kvs: Vec<KVPair> = keys
            .into_iter()
            .filter_map(|key| match self.store.get(&key) {
                Some(value) => Some((key, value.clone())),
                None => None,
            })
            .collect();

        Ok(kvs)
    }

    async fn list(&self) -> Result<Vec<KVPair>> {
        let kvs: Vec<KVPair> = self
            .store
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        Ok(kvs)
    }

    async fn remove(&mut self, keys: Vec<String>) -> Result<()> {
        for key in keys.iter() {
            if let Some(v) = self.store.get(key) {
                let confirm_msg = format!("Remove {key} ({v}) from the store?");
                if utils::confirm_entry(&confirm_msg) {
                    self.store.remove(key);
                }
            }
        }

        utils::write_store(&self.store, &self.loc);

        Ok(())
    }
    async fn clear(&mut self) -> Result<()> {
        let confirm_msg = "Are you sure you want to clear the cache of all contents?";
        if utils::confirm_entry(&confirm_msg) {
            self.store.clear();
        }

        utils::write_store(&self.store, &self.loc);

        Ok(())
    }

    async fn purge(&mut self) -> Result<()> {
        let confirm_msg =
            "Are you sure you want to remove the .safirstore directory and ALL contents?";
        let ws = utils::load_safir_workspace();
        if utils::confirm_entry(&confirm_msg) {
            utils::purge_directory(ws);
            std::process::exit(0);
        }

        Ok(())
    }

    fn get_config(&self) -> SafirConfig {
        self.config.clone()
    }
}
