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
    pub headless_mode: Option<bool>,
}

impl SafirConfig {
    pub fn new() -> Self {
        let mut cfg = Self::default();
        cfg.headless_mode = Some(false);
        cfg
    }

    pub async fn load(cfg_path: impl AsRef<Path>) -> Result<Self> {
        let cfg_str = fs::read_to_string(cfg_path).await?;
        let mut cfg: SafirConfig =
            serde_json::from_str(&cfg_str).expect("unable to deserialize safir config");

        // Backwards compat with older Safir configs
        if cfg.headless_mode.is_none() {
            cfg.headless_mode = Some(false);
        }

        Ok(cfg)
    }

    pub async fn write(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self).expect("unable to serialize safir config");
        let mut file = fs::File::create(&self.config_path).await?;
        file.write_all(data.as_bytes()).await?;

        Ok(())
    }

    pub fn get_headless_mode(&self) -> bool {
        if self.headless_mode.is_some() {
            self.headless_mode.unwrap()
        } else {
            false
        }
    }
}
