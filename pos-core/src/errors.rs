use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("db error: {0}")]
    Db(String),
    #[error("blockchain error: {0}")]
    Chain(String),
    #[error("internal error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Chain(_) => StatusCode::BAD_GATEWAY,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let msg = self.to_string();
        (code, Json(ErrorBody { error: msg })).into_response()
    }
}
