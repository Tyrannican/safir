pub mod file_store;

use crate::utils;

use std::path::Path;

use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

pub trait SafirStore {
    async fn add(key: String, value: String) -> Result<()>;
    async fn get(keys: Vec<String>) -> Result<Vec<String>>;
    async fn list(keys: Vec<String>) -> Result<Vec<String>>;
    async fn remove(keys: Vec<String>) -> Result<()>;
    async fn clear() -> Result<()>;
    async fn purge() -> Result<()>;
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
    pub fn load(fp: impl AsRef<Path>) -> Result<Self> {
        let fp = fp.as_ref().join("safirstore.cfg");
        let contents = std::fs::read_to_string(&fp).context("loading safir config")?;
        serde_json::from_str(&contents).context("deserializing safir config")
    }
}

pub fn init_safir() {
    // 1. Create .safirstore dir
    let _ws = utils::create_safir_workspace();

    // 2. Load / Create config
    // 3. Load / Create stores
}
