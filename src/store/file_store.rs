use crate::utils;
use std::{collections::HashMap, fs, path::PathBuf};

pub struct KVStore {
    pub path: PathBuf,
    pub file: PathBuf,
    pub store: HashMap<String, String>,
}

impl KVStore {
    pub fn init_safir() -> Self {
        match dirs::home_dir() {
            Some(home) => {
                let working_dir = home.join(".safirstore");
                fs::create_dir_all(&working_dir).expect("unable to create main directory");

                let store_path = working_dir.join("safirstore.json");
                let store = if store_path.exists() {
                    utils::load_store(&store_path)
                } else {
                    let store = HashMap::new();
                    utils::write_store(&store, &store_path);
                    store
                };

                return Self {
                    path: working_dir,
                    file: store_path,
                    store,
                };
            }
            None => {
                eprintln!("unable to obtain home directory path!");
                std::process::exit(-1);
            }
        }
    }

    pub fn add(&mut self, key: String, value: String) {
        if let Some(v) = self.store.get(&key) {
            let confirm_msg = format!("Key {key} already exists ({v}), Replace?");
            if utils::confirm_entry(&confirm_msg) {
                self.store.insert(key, value);
            }
        } else {
            self.store.insert(key, value);
        }
    }

    pub fn get(&self, keys: Vec<String>) {
        for key in keys.iter() {
            if let Some(value) = self.store.get(key) {
                utils::display_kv(key, value);
            }
        }
    }

    pub fn list(&self) {
        for (key, value) in self.store.iter() {
            utils::display_kv(key, value);
        }
    }

    pub fn remove(&mut self, keys: Vec<String>) {
        for key in keys.iter() {
            if let Some(v) = self.store.get(key) {
                let confirm_msg = format!("Remove {key} ({v}) from the store?");
                if utils::confirm_entry(&confirm_msg) {
                    self.store.remove(key);
                }
            }
        }
    }

    pub fn custom_display(&self, display_cmd: &str, keys: Vec<String>) {
        for key in keys.iter() {
            if let Some(value) = self.store.get(key) {
                println!("{display_cmd} {key}=\"{value}\"");
            }
        }
    }

    pub fn clear(&mut self) {
        let confirm_msg = "Are you sure you want to clear the cache of all contents?";
        if utils::confirm_entry(&confirm_msg) {
            self.store.clear();
        }
    }

    pub fn purge(&mut self) {
        let confirm_msg =
            "Are you sure you want to remove the .safirstore directory and ALL contents?";
        if utils::confirm_entry(&confirm_msg) {
            utils::purge_directory(self.path.clone());
            std::process::exit(0);
        }
    }
}
