use std::time::Duration;

use clap::Args;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::log::LevelFilter;

use crate::error::Result;

/// Define database connection information.
#[derive(Debug, Args, Default, Clone)]
pub struct DatabaseCommands {
    /// Database connection URL i.e `postgresql://username:Password@host:port/database`
    #[clap(long)]
    pub uri: String,

    /// Database connection timeout in seconds
    /// How long before the connection is aborted
    #[clap(short = 't', long, default_value_t = 5)]
    pub connect_timeout: u64,

    /// Idle timeout in seconds.
    /// If a connection is idle for this amount it will be closed
    #[clap(short = 'i', long, default_value_t = 5)]
    pub idle_timeout: u64,

    /// Maximum number of connections for the database.
    #[clap(short = 'd', long, default_value_t = 1)]
    pub max_connections: u32,

    /// Minimum number of connections for the database.
    #[clap(short = 'n', long, default_value_t = 1)]
    pub min_connections: u32,

    /// Should you enable logging of sql statements. Default is false.
    #[clap(long, default_value_t = false)]
    pub log: bool,
}

impl DatabaseCommands {
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
