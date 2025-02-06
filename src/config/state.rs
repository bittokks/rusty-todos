use sqlx::PgPool;

use crate::{
    app::{App, Commands},
    error::{Error, Result},
};

#[derive(Clone)]
pub struct AppContext {
    pub db: PgPool,
    pub config: App,
}

impl AppContext {
    pub async fn new(app: &App) -> Result<Self> {
        if let Commands::Database(dbc) = &app.commands {
            let db = dbc.connection_pool().await?;

            Ok(Self {
                db,
                config: app.clone(),
            })
        } else {
            Err(Error::InternalServerError.into())
        }
    }
}
