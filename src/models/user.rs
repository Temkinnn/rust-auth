pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "user_role")]
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

pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
}
