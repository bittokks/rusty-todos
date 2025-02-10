use std::time::Duration;

use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::log::LevelFilter;

use crate::error::Result;

/// Define database connection information.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub uri: String,

    /// Milliseconds
    pub connect_timeout: u64,

    /// Milliseconds
    pub idle_timeout: u64,

    pub max_connections: u32,

    pub min_connections: u32,

    pub log: bool,
}

impl DatabaseConfig {
    pub async fn connection_pool(&self) -> Result<PgPool> {
        let options = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(self.connect_timeout))
            .idle_timeout(Duration::from_secs(self.idle_timeout))
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_slow_level(LevelFilter::Debug);

        let pool = options.connect_lazy(&self.uri)?;

        Ok(pool)
    }
}
