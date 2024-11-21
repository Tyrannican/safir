use crate::{
    store::{config::SafirConfig, SafirStore},
    utils::{self, KVPair},
};

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct KVStore {
    loc: PathBuf,
    environment: String,
    store: HashMap<String, HashMap<String, String>>,
    config: SafirConfig,
}

impl KVStore {
    pub fn load(ws: PathBuf, config: SafirConfig) -> Self {
        let store_path = ws.join("safirstore.json");
        let safir = if store_path.exists() {
            let store = Self::load_store(&store_path, &config);
            Self {
                loc: store_path,
                environment: config.environment.clone(),
                config,
                store,
            }
        } else {
            let mut store = HashMap::new();
            store.insert(config.environment.clone(), HashMap::new());

            Self {
                loc: store_path,
                store,
                environment: config.environment.clone(),
                config,
            }
        };

        safir.write_store();
        safir
    }

    /// Loads the store from disk
    /// This is stupid having to reload the map but it will allow users that had
    /// the old format to port over to the new format seamlessly
    pub fn load_store(
        path: impl AsRef<Path>,
        config: &SafirConfig,
    ) -> HashMap<String, HashMap<String, String>> {
        let contents = std::fs::read_to_string(path.as_ref()).expect("unable to store contents");

        let store: HashMap<String, Value> =
            serde_json::from_str(&contents).expect("unable to deserialize contents");

        // If they're all objects then this is the new format, load appropriately
        if store.values().all(|v| v.is_object()) {
            return serde_json::from_str::<HashMap<String, HashMap<String, String>>>(&contents)
                .unwrap();
        }

        // Old map (no environment) - Load and create new environment
        let old_map = serde_json::from_str::<HashMap<String, String>>(&contents).unwrap();
        let mut new_map = HashMap::new();
        new_map.insert(config.environment.clone(), HashMap::new());

        let env = new_map.get_mut(config.environment.as_str()).unwrap();
        for (key, value) in old_map.into_iter() {
            env.insert(key, value);
        }

        new_map
    }

    /// Writes the store to disk
    pub fn write_store(&self) {
        use std::io::Write;
        let str_store =
            serde_json::to_string_pretty(&self.store).expect("unable to serialize store contents");

        let mut file = std::fs::File::create(&self.loc).expect("unable to get file handle");

        file.write_all(str_store.as_bytes())
            .expect("unable to write store out to disk");
    }

    pub fn get_environment(&mut self) -> &mut HashMap<String, String> {
        self.store
            .entry(self.environment.clone())
            .or_insert(HashMap::new());

        self.store.get_mut(&self.environment).unwrap()
    }
}

#[async_trait]
impl SafirStore for KVStore {
    async fn add(&mut self, key: String, value: String) -> Result<()> {
        let env = self.get_environment();
        if let Some(v) = env.get(&key) {
            let confirm_msg = format!("Key {key} already exists ({v}), Replace?");
            if utils::confirm_entry(&confirm_msg) {
                env.insert(key, value);
            }
        } else {
            env.insert(key, value);
        }

        self.write_store();

        Ok(())
    }

    async fn get(&self, keys: Vec<String>) -> Result<Vec<KVPair>> {
        let inner = match self.store.get(&self.environment) {
            Some(inner) => inner,
            None => return Ok(vec![]),
        };

        let kvs: Vec<KVPair> = keys
            .into_iter()
            .filter_map(|key| match inner.get(&key) {
                Some(value) => Some((key, value.clone())),
                None => None,
            })
            .collect();

        Ok(kvs)
    }

    async fn list(&self) -> Result<Vec<KVPair>> {
        let inner = match self.store.get(&self.environment) {
            Some(inner) => inner,
            None => return Ok(vec![]),
        };

        let kvs: Vec<KVPair> = inner
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        Ok(kvs)
    }

    async fn remove(&mut self, keys: Vec<String>) -> Result<()> {
        let inner = self.get_environment();
        for key in keys.iter() {
            if let Some(v) = inner.get(key) {
                let confirm_msg = format!("Remove {key} ({v}) from the store?");
                if utils::confirm_entry(&confirm_msg) {
                    inner.remove(key);
                }
            }
        }

        self.write_store();

        Ok(())
    }
    async fn clear(&mut self) -> Result<()> {
        let inner = self.get_environment();
        let confirm_msg = "Are you sure you want to clear the cache of all contents?";
        if utils::confirm_entry(&confirm_msg) {
            inner.clear();
        }

        self.write_store();

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

    async fn environments(&self) -> Result<Vec<String>> {
        Ok(self.store.keys().map(|e| e.to_string()).collect())
    }

    fn get_config(&self) -> SafirConfig {
        self.config.clone()
    }
}
