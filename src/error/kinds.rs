use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("{0}")]
    EntityAlreadyExists(String),
    #[error("Entity not found")]
    EntityNotFound,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl ModelError {
    pub fn response(&self) -> Response {
        let (code, message) = match self {
            Self::EntityAlreadyExists(e) => (StatusCode::CONFLICT, e.to_string()),
            Self::EntityNotFound => (StatusCode::NOT_FOUND, "Entity not found".into()),
            Self::Sqlx(_e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong on our end".to_string(),
            ),
        };

        let body = Json(json!({"message": message}));

        (code, body).into_response()
    }
}

pub type ModelResult<T> = std::result::Result<T, ModelError>;

#[derive(Debug, thiserror::Error, Clone)]
pub enum AuthError {
    #[error("Login session has expired")]
    ExpiredCredentials,
    #[error("Credentials missing from HTTP Request header")]
    MissingCredentials,
    #[error("Invalid username or password")]
    WrongCredentials,
}

impl AuthError {
    pub fn response(&self) -> Response {
        let (status, message) = match self {
            Self::ExpiredCredentials => {
                (StatusCode::UNAUTHORIZED, "Session has expired. Login again")
            }
            Self::MissingCredentials => (StatusCode::UNAUTHORIZED, "Login to continue"),
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong username or password"),
        };

        let body = Json(json!({"message": message}));

        (status, body).into_response()
    }
}

pub type AuthResult<T> = std::result::Result<T, AuthError>;
