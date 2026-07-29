use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error")]
    Database(#[from] sqlx::Error),

    #[error("Internal Server Error")]
    Password(#[from] argon2::password_hash::Error),

    #[error("Internal Server Error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

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
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Password(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Jwt(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::AlreadyExists => StatusCode::CONFLICT,
            AppError::Unathorized => StatusCode::UNAUTHORIZED,
            AppError::InvalidCredentials => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!({
            "error": self.to_string()
        }))
    }
}
