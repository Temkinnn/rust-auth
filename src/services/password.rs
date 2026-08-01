use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{errors::AppError, types::AppResult};

pub struct PasswordService(Argon2<'static>);

impl PasswordService {
    pub fn new() -> Self {
        let argon = Argon2::default();
        Self(argon)
    }

    pub async fn hash_password(&self, password: &str) -> AppResult<String> {
        let hasher = self.0.clone();
        let password_owned = password.to_string();

        let password_hash = tokio::task::spawn_blocking(move || -> AppResult<String> {
            let salt = SaltString::generate(OsRng);

            let hash = hasher
                .hash_password(password_owned.as_bytes(), &salt)
                .map_err(|_| AppError::Internal)?;

            Ok(hash.to_string())
        })
        .await
        .map_err(|_| AppError::Internal)??;

        Ok(password_hash)
    }

    pub async fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        let hasher = self.0.clone();
        let password_owned = password.to_string();
        let hash_owned = hash.to_string();

        let verified = tokio::task::spawn_blocking(move || -> AppResult<bool> {
            let parsed_hash = PasswordHash::new(&hash_owned)?;
            let result = hasher
                .verify_password(password_owned.as_bytes(), &parsed_hash)
                .is_ok();

            Ok(result)
        })
        .await
        .map_err(|_| AppError::Internal)??;

        Ok(verified)
    }
}
