pub mod db_store;
pub mod file_store;

use crate::utils;

use std::path::Path;

use anyhow::{Context, Result};
use clap::ValueEnum;
use db_store::SqliteStore;
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

pub async fn init_safir() -> Result<impl SafirStore> {
    let ws = utils::create_safir_workspace();
    let cfg = SafirConfig::load(&ws).expect("can't load safir config");

    match cfg.mode {
        SafirMode::File => Ok(KVStore::load(ws)),
        SafirMode::Database => unimplemented!("fix this"),
    }
}
