use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SafirConfig {
    #[serde(skip)]
    pub filepath: PathBuf,

    pub environment: String,
}

impl SafirConfig {
    pub fn load(workdir: impl AsRef<Path>) -> Result<Self> {
        let fp = workdir.as_ref().join("safirstore.cfg");
        if !fp.exists() {
            let cfg = Self {
                filepath: fp,
                environment: "default".to_string(),
            };
            cfg.write().context("writing config out")?;

            return Ok(cfg);
        }
        let contents = std::fs::read_to_string(&fp).context("loading safir config")?;
        let mut cfg: SafirConfig =
            serde_json::from_str(&contents).context("deserializing safir config")?;

        cfg.filepath = fp;

        Ok(cfg)
    }

    pub fn write(&self) -> Result<()> {
        use std::io::Write;

        let contents = serde_json::to_string_pretty(&self).context("serializing config")?;
        let mut fd =
            std::fs::File::create(&self.filepath).context("creating config file descriptor")?;

        fd.write_all(contents.as_bytes())
            .context("writing config contents")?;

        Ok(())
    }
}
