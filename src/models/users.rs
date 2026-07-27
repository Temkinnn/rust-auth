pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

pub enum Role {
    Admin,
    User,
}

pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UpdateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}
