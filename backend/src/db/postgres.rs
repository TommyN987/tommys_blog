use std::str::FromStr;

use anyhow::Context;
use sqlx::{PgPool, postgres::PgConnectOptions};
use tracing::{debug, info, instrument};

#[derive(Debug, Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    #[instrument(name = "postgres_init", skip(path))]
    pub async fn try_new(path: &str) -> Result<Self, anyhow::Error> {
        debug!("Connecting to PostgreSQL with URL: {}", path);

        let pool = PgPool::connect_with(
            PgConnectOptions::from_str(path)
                .with_context(|| format!("Invalid database path: {}", path))?,
        )
        .await
        .with_context(|| format!("Failed to open database at {}", path))?;

        info!("✅ Connected to PostgreSQL");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
