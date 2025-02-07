use argon2::password_hash::Error as PasswordHashError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T, E = Report> = color_eyre::Result<T, E>;

pub struct Report(color_eyre::Report);

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for Report
where
    E: Into<color_eyre::Report>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Axum(#[from] axum::Error),
    #[error("{0}")]
    Argon(argon2::Error),
    #[error("{0}")]
    ArgonPasswordHash(argon2::password_hash::Error),
    #[error("{0}")]
    Config(#[from] config::ConfigError),
    #[error("{0}")]
    ConfigFile(String),
    #[error("{0}")]
    EntityAlreadyExists(String),
    #[error("Entity not found.")]
    EntityNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("{0}")]
    InvalidCredentials(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Page not found")]
    NotFound,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    WrongCredentials(String),
}

impl Error {
    fn response(&self) -> Response {
        let (status, message) = match self {
            Self::NotFound | Self::EntityNotFound => {
                (StatusCode::NOT_FOUND, "Page not found".to_string())
            }
            Self::EntityAlreadyExists(e) => (StatusCode::CONFLICT, e.into()),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            Self::WrongCredentials(e) | Self::InvalidCredentials(e) => {
                (StatusCode::UNAUTHORIZED, e.to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        (status, message).into_response()
    }
}

impl IntoResponse for Report {
    fn into_response(self) -> Response {
        let err = &self.0;
        let err_string = format!("{:?}", err);

        tracing::error!("{err_string}");

        if let Some(e) = err.downcast_ref::<Error>() {
            return e.response();
        }

        // Fallback error
        //
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
            .into_response()
    }
}

impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Self::Argon(err)
    }
}

impl From<PasswordHashError> for Error {
    fn from(err: PasswordHashError) -> Self {
        match err {
            PasswordHashError::Password => {
                Self::InvalidCredentials("Invalid login credentials".to_string())
            }
            _ => Self::ArgonPasswordHash(err),
        }
    }
}
