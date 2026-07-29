use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::{
    env::Env,
    models::token::{Claims, Tokens},
    types::{AppResult, Id},
};

pub struct TokenService {
    secret: String,
    access_token_expires: u64,
    refresh_token_expires: u64,
}

impl TokenService {
    pub fn new() -> Self {
        let env = Env::init();
        Self {
            secret: env.jwt_secret,
            access_token_expires: (env.access_token_expires_mins * 60) as u64, // mins to seconds
            refresh_token_expires: (env.refresh_token_expires_days * 24 * 60 * 60) as u64, // days to seconds
        }
    }

    pub fn generate_access_token(&self, id: Id) -> AppResult<String> {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Current Time")
            .as_secs()
            + self.access_token_expires;

        let claims = Claims {
            sub: id,
            exp: exp as usize,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?)
    }

    fn generate_refresh_token(&self, id: Id) -> AppResult<String> {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Current Time")
            .as_secs()
            + self.refresh_token_expires;

        let claims = Claims {
            sub: id,
            exp: exp as usize,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
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

    pub fn verify_token(&self, token: String) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
