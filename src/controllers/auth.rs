use std::sync::Arc;

use axum::{
    body::Body, extract::State, http::StatusCode, response::Response, routing::post, Json, Router,
};
use serde_json::json;

use crate::{
    config::state::AppContext,
    error::Result,
    models::users::{FilteredUser, RegisterUser, User},
};

async fn register(
    State(ctx): State<Arc<AppContext>>,
    Json(dto): Json<RegisterUser<'static>>,
) -> Result<Response> {
    let user = User::register(&ctx.db, &dto).await?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(json!(FilteredUser::from(user)).to_string()))?)
}

pub fn routes() -> Router<Arc<AppContext>> {
    Router::new().route("/register", post(register))
}
