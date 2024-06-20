pub mod file_store;

use crate::utils;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::ValueEnum;
use file_store::KVStore;
use serde::{Deserialize, Serialize};

pub trait SafirStore {
    async fn add(&mut self, key: String, value: String) -> Result<()>;
    async fn get(&self, keys: Vec<String>) -> Result<()>;
    async fn list(&self) -> Result<()>;
    async fn remove(&mut self, keys: Vec<String>) -> Result<()>;
    async fn clear(&mut self) -> Result<()>;
    async fn purge(&mut self) -> Result<()>;
}

#[derive(ValueEnum, Default, Debug, Copy, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SafirMode {
    #[default]
    File,
    Database,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SafirConfig {
    mode: SafirMode,
}

impl SafirConfig {
    pub fn load(workdir: impl AsRef<Path>) -> Result<Self> {
        let fp = workdir.as_ref().join("safirstore.cfg");
        if !fp.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&fp).context("loading safir config")?;
        serde_json::from_str(&contents).context("deserializing safir config")
    }
}

pub fn init_safir() -> Result<impl SafirStore> {
    // 1. Create .safirstore dir
    let ws = utils::create_safir_workspace();

    // 2. Load / Create config
    let cfg = SafirConfig::load(&ws).expect("can't load safir config");

    // 3. Load / Create stores
    let store = match cfg.mode {
        SafirMode::File => KVStore::load(ws),
        _ => unimplemented!("not yet on db work"),
    };

    Ok(store)
}
