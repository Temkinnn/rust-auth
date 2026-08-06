use serde::Deserialize;

use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct LoginCredentials {
    #[validate(email(message = "Please provide proper email adress"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegistrationCredentials {
    #[validate(length(min = 4, message = "Password must be at least 4 characters long"))]
    pub username: String,
    #[validate(email(message = "Please provide proper email adress"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}
