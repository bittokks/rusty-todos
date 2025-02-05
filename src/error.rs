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
    #[error("Page not found")]
    NotFound,
    #[error("Internal server error")]
    InternalServerError,
}

impl Error {
    fn response(&self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Page not found".to_string()),
            Self::InternalServerError => (
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
