use crate::utils;
use std::{fs, path::PathBuf, collections::HashMap};

pub struct Store {
    pub path: PathBuf,
    pub file: PathBuf,
    pub store: HashMap<String, String>
}

impl Store {
    pub fn init_safir() -> Self {
        match dirs::home_dir() {
            Some(home) => {
                let working_dir = home.join(".safirstore");
                fs::create_dir_all(&working_dir)
                    .expect("unable to create main directory");

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

    }

    pub fn get(&self, keys: Vec<String>) {

    }

    pub fn list(&self) {

    }

    pub fn remove(&mut self, keys: Vec<String>) {
        
    }

    pub fn custom_display(&self, display_cmd: &str, keys: Vec<String>) {

    }

    pub fn clear(&mut self) {

    }

    pub fn purge(&mut self) {

    }
}
