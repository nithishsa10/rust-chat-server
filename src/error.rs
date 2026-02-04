use axum::{
    Json, 
    http::{StatusCode}, 
    response::{IntoResponse, Response}
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Unatuhorized Error: {0}")]
    Unauthorized(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Conflict Error: {0}")]
    Conflict(String),

    #[error("Forbidden Error: {0}")]
    Forbidden(String),

    #[error("Database Error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Internal Error: {0}")]
    Internal(String),

     #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("Rate limit exceeded")]
    RateLimited,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            AppError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "TOO_MANY_REQUESTS", "Rate limit exceeded".into()),
            AppError::Internal(_)
            | AppError::Database(_)
            | AppError::Redis(_)
            | AppError::Jwt(_)
            | AppError::Bcrypt(_)        => (StatusCode::INTERNAL_SERVER_ERROR,  "INTERNAL_SERVER_ERROR", "Internal server error".into()),
        };

        tracing::error!("AppError:{}", self);

        (
            status,
            Json(json!({
                "type": "error",
                "code": code,
                "message": message
            })),
        ).into_response()

    }
}

pub type Result<T> = std::result::Result<T, AppError>;