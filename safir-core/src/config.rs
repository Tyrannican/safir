use serde::{Deserialize, Serialize};
use std::{
    io::Result,
    path::{Path, PathBuf},
};
use tokio::{fs, io::AsyncWriteExt};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SafirConfig {
    #[serde(skip)]
    pub root_path: PathBuf,
    #[serde(skip)]
    pub config_path: PathBuf,
    pub memcache_pid: Option<u32>,
}

impl SafirConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn load(cfg_path: impl AsRef<Path>) -> Result<Self> {
        let cfg = fs::read_to_string(cfg_path).await?;
        Ok(serde_json::from_str(&cfg).expect("unable to deserialize safir config"))
    }

    pub async fn write(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self).expect("unable to serialize safir config");
        let mut file = fs::File::create(&self.config_path).await?;
        file.write_all(data.as_bytes()).await?;

        Ok(())
    }
}
