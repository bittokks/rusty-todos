use sqlx::PgPool;

use crate::error::Result;

use super::app::AppConfig;

#[derive(Clone)]
pub struct AppContext {
    pub db: PgPool,
    pub config: AppConfig,
}

impl AppContext {
    pub async fn new(cfg: &AppConfig) -> Result<Self> {
        let db = cfg.database.connection_pool().await?;

        Ok(Self {
            db,
            config: cfg.clone(),
        })
    }
}
