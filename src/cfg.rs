use serde::{Deserialize, Serialize};
use std::{io::Result, path::Path};
use tokio::{fs, io::AsyncWriteExt};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SafirConfig {
    pub memcache_pid: Option<u32>,
    pub memcache_flag: bool,
}

impl SafirConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn load(cfg_path: impl AsRef<Path>) -> Result<Self> {
        let cfg = fs::read_to_string(cfg_path).await?;
        Ok(serde_json::from_str(&cfg).expect("unable to deserialize safir config"))
    }

    pub fn pid(mut self, pid: Option<u32>) -> Self {
        self.memcache_pid = pid;
        self
    }

    pub fn set_memcache(mut self, state: bool) -> Self {
        self.memcache_flag = state;
        self
    }

    pub async fn write(&self, cfg_path: impl AsRef<Path>) -> Result<()> {
        let data = serde_json::to_string_pretty(&self).expect("unable to serialize safir config");
        let mut file = fs::File::create(cfg_path).await?;
        file.write_all(data.as_bytes()).await?;

        Ok(())
    }
}
