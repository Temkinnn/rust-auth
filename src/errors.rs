use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use redis::RedisError;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error")]
    Database(#[from] sqlx::Error),

    #[error("Storage Error")]
    Redis(#[from] RedisError),

    #[error("{0}")]
    Password(#[from] argon2::password_hash::Error),

    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Internal Server Error")]
    Internal,

    #[error("Resource not found")]
    NotFound,

    #[error("Resource already exists")]
    AlreadyExists,

    #[error("Unauthorized")]
    Unathorized,

    #[error("Invalid credentials")]
    InvalidCredentials,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Password(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Jwt(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::AlreadyExists => StatusCode::CONFLICT,
            Self::Unathorized => StatusCode::UNAUTHORIZED,
            Self::InvalidCredentials => StatusCode::BAD_REQUEST,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!({
            "error": self.to_string()
        }))
    }
}
