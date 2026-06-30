use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("email already registered")]
    EmailTaken,

    #[error("invalid email or password")]
    InvalidCredentials,

    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("password hashing error")]
    Hash(String),

    #[error("token error")]
    Token(#[from] jsonwebtoken::errors::Error),

    #[error("unauthorized")]
    Unauthorized,

    #[error("internal server error")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::EmailTaken => (StatusCode::CONFLICT, self.to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Database(e) => {
                tracing::error!("database error: {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
            AppError::Hash(e) => {
                tracing::error!("hash error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
            AppError::Token(e) => {
                tracing::error!("token error: {e:?}");
                (StatusCode::UNAUTHORIZED, "invalid or expired token".to_string())
            }
            AppError::Internal(e) => {
                tracing::error!("internal error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
