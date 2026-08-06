use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "role")]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserDto {
    #[validate(length(min = 4, message = "Password must be at least 4 characters long"))]
    pub username: String,
    #[validate(email(message = "Please provide proper email adress"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    pub role: Option<Role>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserDto {
    #[validate(length(min = 4, message = "Password must be at least 4 characters long"))]
    pub username: Option<String>,
    #[validate(email(message = "Please provide proper email adress"))]
    pub email: Option<String>,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: Option<String>,
    pub role: Option<Role>,
}
