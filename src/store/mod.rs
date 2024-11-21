pub mod config;
pub mod db_store;
pub mod file_store;

use crate::utils::{load_safir_workspace, KVPair};
use config::{SafirConfig, SafirMode};

use anyhow::Result;
use async_trait::async_trait;
use db_store::SqliteStore;
use file_store::KVStore;

#[async_trait]
pub trait SafirStore {
    async fn add(&mut self, key: String, value: String) -> Result<()>;
    async fn get(&self, keys: Vec<String>) -> Result<Vec<KVPair>>;
    async fn list(&self) -> Result<Vec<KVPair>>;
    async fn remove(&mut self, keys: Vec<String>) -> Result<()>;
    async fn clear(&mut self) -> Result<()>;
    async fn purge(&mut self) -> Result<()>;
    async fn environments(&self) -> Result<Vec<String>>;
    fn get_config(&self) -> SafirConfig;
}

pub async fn init_safir() -> Result<Box<dyn SafirStore>> {
    let ws = load_safir_workspace();
    let cfg = SafirConfig::load(&ws).expect("can't load safir config");

    match cfg.mode {
        SafirMode::File => Ok(Box::new(KVStore::load(ws, cfg))),
        SafirMode::Database => Ok(Box::new(SqliteStore::load(ws, cfg).await?)),
    }
}
