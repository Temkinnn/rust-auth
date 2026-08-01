use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "role")]
pub enum Role {
    Admin,
    User,
}

pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<Role>,
}

pub struct CreateUserRepoDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<Role>,
}

pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
}
