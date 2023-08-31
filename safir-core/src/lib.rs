use anyhow::Result;
use async_trait::async_trait;
use config::SafirConfig;
use disk::SafirStore;
use mem::SafirMemcache;

pub mod config;
pub mod disk;
pub mod mem;
pub mod utils;

use std::any::Any;

#[async_trait]
pub trait SafirEngine {
    async fn add_entry(&mut self, key: String, value: String) -> Result<()>;
    async fn get_entry(&self, key: String) -> Result<()>;
    async fn remove_entry(&mut self, keys: Vec<String>) -> Result<()>;
    async fn clear_entries(&mut self) -> Result<()>;
    async fn set_commands(&mut self, prefix: &str, keys: &Vec<String>);
    fn to_type(&self) -> &dyn Any;
}

pub struct Safir {
    pub engine: Box<dyn SafirEngine>,
    pub config: SafirConfig,
}

#[derive(Debug, PartialEq)]
pub enum SafirEngineType {
    Store,
    Memcache,
}

impl Safir {
    pub async fn new(engine_type: SafirEngineType) -> Result<Self> {
        let config = utils::init().await?;
        match engine_type {
            SafirEngineType::Store => {
                let e = crate::disk::SafirStore::new(&config).await?;
                Ok(Self {
                    engine: Box::new(e),
                    config,
                })
            }
            SafirEngineType::Memcache => {
                let e = crate::mem::SafirMemcache::new(&config).await?;
                Ok(Self {
                    engine: Box::new(e),
                    config,
                })
            }
        }
    }

    pub async fn add_entry(&mut self, key: String, value: String) -> Result<()> {
        self.engine.add_entry(key, value).await?;
        Ok(())
    }

    pub async fn get_entry(&self, key: String) -> Result<()> {
        self.engine.get_entry(key).await?;
        Ok(())
    }

    pub async fn remove_entry(&mut self, keys: Vec<String>) -> Result<()> {
        self.engine.remove_entry(keys).await?;
        Ok(())
    }

    pub async fn clear_entries(&mut self) -> Result<()> {
        self.engine.clear_entries().await?;
        Ok(())
    }

    pub async fn set_commands(&mut self, prefix: &str, keys: &Vec<String>) {
        self.engine.set_commands(prefix, keys).await;
    }

    pub fn as_safir_store(&self) -> &SafirStore {
        self.engine
            .to_type()
            .downcast_ref::<SafirStore>()
            .expect("unable to get Safir store type")
    }

    pub fn as_safir_memcache(&self) -> &SafirMemcache {
        self.engine
            .to_type()
            .downcast_ref::<SafirMemcache>()
            .expect("unable to get Safir memcache type")
    }
}
