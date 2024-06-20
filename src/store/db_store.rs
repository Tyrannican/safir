use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};

use std::path::PathBuf;
use std::str::FromStr;

use crate::store::SafirStore;

#[derive(Debug, Clone)]
pub struct SqliteStore {
    pool: SqlitePool,
}

impl SqliteStore {
    pub(crate) async fn load(wd: PathBuf) -> Result<Self> {
        let lead = PathBuf::from("sqlite:/");
        let db_name = lead.join(wd).join("safirstore.db");

        let connect_opts = SqliteConnectOptions::from_str(db_name.to_str().unwrap())?
            .optimize_on_close(true, None)
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs_f64(0.1))
            .connect_with(connect_opts)
            .await
            .context("creating database")?;

        Self::setup_db(&pool).await?;
        Ok(Self { pool })
    }

    async fn setup_db(pool: &SqlitePool) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .context("running database migrations")?;

        Ok(())
    }
}

impl SafirStore for SqliteStore {
    async fn add(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }
    async fn get(&self, keys: Vec<String>) -> Result<()> {
        Ok(())
    }
    async fn list(&self) -> Result<()> {
        Ok(())
    }
    async fn remove(&mut self, keys: Vec<String>) -> Result<()> {
        Ok(())
    }
    async fn clear(&mut self) -> Result<()> {
        Ok(())
    }
    async fn purge(&mut self) -> Result<()> {
        Ok(())
    }
}
