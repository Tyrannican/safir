use crate::{store::SafirStore, utils};
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

pub struct KVStore {
    pub path: PathBuf,
    pub store: HashMap<String, String>,
}

impl KVStore {
    pub fn load(ws: PathBuf) -> Self {
        let store_path = ws.join("safirstore.json");
        let store = if store_path.exists() {
            utils::load_store(&store_path)
        } else {
            let store = HashMap::new();
            utils::write_store(&store, &store_path);
            store
        };

        Self { path: ws, store }
    }

    pub fn custom_display(&self, display_cmd: &str, keys: Vec<String>) {
        for key in keys.iter() {
            if let Some(value) = self.store.get(key) {
                println!("{display_cmd} {key}=\"{value}\"");
            }
        }
    }
}

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

        Ok(())
    }

    async fn get(&self, keys: Vec<String>) -> Result<()> {
        for key in keys.iter() {
            if let Some(value) = self.store.get(key) {
                utils::display_kv(key, value);
            }
        }

        Ok(())
    }

    async fn list(&self) -> Result<()> {
        for (key, value) in self.store.iter() {
            utils::display_kv(key, value);
        }

        Ok(())
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

        Ok(())
    }
    async fn clear(&mut self) -> Result<()> {
        let confirm_msg = "Are you sure you want to clear the cache of all contents?";
        if utils::confirm_entry(&confirm_msg) {
            self.store.clear();
        }

        Ok(())
    }

    async fn purge(&mut self) -> Result<()> {
        let confirm_msg =
            "Are you sure you want to remove the .safirstore directory and ALL contents?";
        if utils::confirm_entry(&confirm_msg) {
            utils::purge_directory(self.path.clone());
            std::process::exit(0);
        }

        Ok(())
    }
}
