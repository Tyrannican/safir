use anyhow::Result;
use async_trait::async_trait;

pub mod config;
pub mod disk;
pub mod mem;
pub mod utils;

#[async_trait]
pub trait SafirEngine {
    async fn add_entry(&mut self, key: String, value: String) -> Result<()>;
    async fn get_entry(&self, key: String) -> Result<()>;
    async fn remove_entry(&mut self, keys: Vec<String>) -> Result<()>;
    async fn clear_entries(&mut self) -> Result<()>;
    async fn set_commands(&mut self, prefix: &str, keys: &Vec<String>);
}

pub struct Safir {
    engine: Box<dyn SafirEngine>,
}

impl Safir {
    pub fn new<S: SafirEngine + 'static>(engine: S) -> Self {
        Self {
            engine: Box::new(engine),
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
}
