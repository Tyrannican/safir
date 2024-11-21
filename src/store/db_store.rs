use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};

use std::path::PathBuf;
use std::str::FromStr;

use crate::{
    store::{config::SafirConfig, SafirStore},
    utils::{confirm_entry, load_safir_workspace, purge_directory, KVPair},
};

#[derive(Debug, Clone)]
pub struct SqliteStore {
    pool: SqlitePool,
    config: SafirConfig,
}

impl SqliteStore {
    pub(crate) async fn load(ws: PathBuf, config: SafirConfig) -> Result<Self> {
        let lead = PathBuf::from("sqlite:/");
        let db_name = lead.join(ws).join("safirstore.db");

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
        Ok(Self { pool, config })
    }

    async fn setup_db(pool: &SqlitePool) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .context("running database migrations")?;

        Ok(())
    }
}

#[async_trait]
impl SafirStore for SqliteStore {
    async fn add(&mut self, key: String, value: String) -> Result<()> {
        sqlx::query("insert into safir(key, value, environment) values(?1, ?2, ?3)")
            .bind(&key)
            .bind(&value)
            .bind(&self.config.environment)
            .execute(&self.pool)
            .await
            .with_context(|| format!("insert {key} - {value} into database"))?;

        Ok(())
    }

    async fn get(&self, keys: Vec<String>) -> Result<Vec<KVPair>> {
        let keys = keys
            .into_iter()
            .map(|k| format!("'{k}'"))
            .collect::<Vec<String>>();

        let query = format!(
            "select * from safir where environment = '{}' and key in ({})",
            &self.config.environment,
            keys.join(", ")
        );
        let results: Vec<KVPair> = sqlx::query_as::<_, KVPair>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(results)
    }

    async fn list(&self) -> Result<Vec<KVPair>> {
        let query = format!(
            "select * from safir where environment = '{}'",
            self.config.environment
        );
        let results: Vec<KVPair> = sqlx::query_as::<_, KVPair>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(results)
    }

    async fn remove(&mut self, keys: Vec<String>) -> Result<()> {
        let keys = keys
            .into_iter()
            .map(|k| format!("'{k}'"))
            .collect::<Vec<String>>();

        let query = format!(
            "delete from safir where environment = '{}' and key in ({})",
            self.config.environment,
            keys.join(", ")
        );
        let _ = sqlx::query_as::<_, KVPair>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(())
    }

    async fn clear(&mut self) -> Result<()> {
        if confirm_entry("Are you sure you want to clear the safirstore?") {
            let query = format!(
                "delete from safir where environment = '{}'",
                self.config.environment
            );
            let _ = sqlx::query_as::<_, KVPair>(&query)
                .fetch_all(&self.pool)
                .await?;
        }

        Ok(())
    }

    async fn purge(&mut self) -> Result<()> {
        let ws = load_safir_workspace();
        let confirm_msg =
            "Are you sure you want to purge the safirstore?\nThis will delete the folder and any data inside!";

        if confirm_entry(&confirm_msg) {
            purge_directory(ws);
        }
        Ok(())
    }

    async fn environments(&self) -> Result<Vec<String>> {
        let query = format!("select distinct environment from safir");
        let result: Vec<String> = sqlx::query_scalar(&query).fetch_all(&self.pool).await?;
        Ok(result)
    }

    fn get_config(&self) -> SafirConfig {
        self.config.clone()
    }
}
