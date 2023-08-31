use std::convert::TryFrom;

use derive_more::Deref;
use sqlx::{pool::PoolConnection, Postgres, Transaction};
use tracing_attributes::instrument;
use url::Url;

use crate::{
    error::{ConnectedHomeError, ConnectedHomeResult},
    util::cli::from_file_or_const,
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PostgresConfig {
    pub hostname: String,
    pub port: u32,
    pub database: String,
    pub max_connections: u32,
    #[serde(deserialize_with = "from_file_or_const")]
    pub username: String,
    #[serde(deserialize_with = "from_file_or_const")]
    pub password: String,
}

impl TryFrom<&PostgresConfig> for Url {
    type Error = ConnectedHomeError;

    fn try_from(config: &PostgresConfig) -> Result<Self, Self::Error> {
        Ok(Url::parse(&format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username, config.password, config.hostname, config.port, config.database
        ))?)
    }
}

#[derive(Deref, Debug)]
pub struct PgPool {
    pool: sqlx::Pool<Postgres>,
}

impl PgPool {
    pub async fn from_config(config: &PostgresConfig) -> ConnectedHomeResult<Self> {
        Ok(Self {
            pool: sqlx::postgres::PgPoolOptions::new()
                .max_connections(config.max_connections)
                .connect(Url::try_from(config)?.as_str())
                .await?,
        })
    }

    #[instrument]
    pub async fn acquire(&self) -> ConnectedHomeResult<PoolConnection<Postgres>> {
        Ok(self.pool.acquire().await?)
    }

    #[instrument]
    pub async fn start_transaction(&self) -> ConnectedHomeResult<Transaction<'_, Postgres>> {
        Ok(self.pool.begin().await?)
    }

    #[instrument]
    pub async fn finalize_transaction(&self, tx: Transaction<'_, Postgres>) -> ConnectedHomeResult<()> {
        Ok(tx.commit().await?)
    }
}
