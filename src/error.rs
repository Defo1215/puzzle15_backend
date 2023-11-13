use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type PuzzleResult<T> = Result<T, PuzzleError>;

#[derive(Error, Debug)]
pub enum PuzzleError {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("db error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("forbidden")]
    Forbidden,
    #[error("jwt_error: {0}")]
    JWT(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for PuzzleError {
    fn into_response(self) -> Response {
        let code = match self {
            PuzzleError::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = self.to_string();
        tracing::error!("{body}");
        (code, body).into_response()
    }
}
