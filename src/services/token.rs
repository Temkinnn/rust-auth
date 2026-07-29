use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

use crate::{
    config::jwt::JwtConfig,
    models::token::{AccessClaims, RefreshClaims, Tokens},
    repositories::token::TokenRepository,
    types::{AppResult, Id},
};

pub struct TokenService {
    jwt: JwtConfig,
    repo: TokenRepository,
}

impl TokenService {
    pub fn new(jwt: JwtConfig, repo: TokenRepository) -> Self {
        Self { jwt, repo }
    }

    pub fn generate_access_token(&self, id: Id) -> AppResult<String> {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Current Time")
            + self.jwt.access_expiration;

        let claims = AccessClaims {
            sub: id,
            exp: exp.as_secs() as usize,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt.secret.as_bytes()),
        )?)
    }

    fn generate_refresh_token(&self, id: Id) -> AppResult<String> {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Current Time")
            + self.jwt.refresh_expiration;

        let claims = RefreshClaims {
            sub: id,
            jti: Uuid::now_v7(),
            exp: exp.as_secs() as usize,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt.secret.as_bytes()),
        )?)
    }

    pub fn generate_tokens(&self, id: Id) -> AppResult<Tokens> {
        let access_token = self.generate_access_token(id)?;
        let refresh_token = self.generate_refresh_token(id)?;

        Ok(Tokens {
            access_token,
            refresh_token,
        })
    }

    pub fn verify_access_token(&self, token: &String) -> AppResult<AccessClaims> {
        let token_data = decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(self.jwt.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub fn verify_refresh_token(&self, token: &String) -> AppResult<RefreshClaims> {
        let token_data = decode::<RefreshClaims>(
            token,
            &DecodingKey::from_secret(self.jwt.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub async fn save_refresh_token(&self, token: String) -> AppResult<()> {
        let verified_token = self.verify_refresh_token(&token)?;

        let exp = self.jwt.refresh_expiration.as_secs();
        self.repo.save_token(verified_token.jti, token, exp).await
    }

    pub async fn delete_refresh_token(&self, jti: Id) -> AppResult<()> {
        self.repo.delete_token(jti).await
    }

    pub async fn update_refresh_token(&self, token: String) -> AppResult<()> {
        let verified_token = self.verify_refresh_token(&token)?;

        let exp = self.jwt.refresh_expiration.as_secs();
        self.repo.update_token(verified_token.jti, token, exp).await
    }
}
