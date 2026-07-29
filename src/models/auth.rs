use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegistrationCredentials {
    pub username: String,
    pub email: String,
    pub password: String,
}
