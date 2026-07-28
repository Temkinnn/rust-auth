use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::types::AppResult;

pub struct PasswordService(Argon2<'static>);

impl PasswordService {
    fn new() -> Self {
        let argon = Argon2::default();
        Self(argon)
    }

    fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(OsRng);

        Ok(self
            .0
            .hash_password(password.as_bytes(), &salt)?
            .to_string())
    }

    async fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(self
            .0
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
