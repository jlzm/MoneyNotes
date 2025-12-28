use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("JWT error: {0}")]
    Jwt(String),
}

impl AppError {
    pub fn code(&self) -> i32 {
        match self {
            AppError::Validation(_) => 10001,
            AppError::Unauthorized => 10002,
            AppError::Forbidden(_) => 10003,
            AppError::NotFound(_) => 10004,
            AppError::Internal(_) => 10005,
            AppError::Conflict(_) => 20001,
            AppError::Database(_) => 10005,
            AppError::Jwt(_) => 20003,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, self.code(), msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.code(), "Unauthorized".to_string()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, self.code(), msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, self.code(), msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, self.code(), msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, self.code(), msg.clone()),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, self.code(), msg.clone()),
            AppError::Jwt(msg) => (StatusCode::UNAUTHORIZED, self.code(), msg.clone()),
        };

        let body = Json(json!({
            "code": code,
            "message": message
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
