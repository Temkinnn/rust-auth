use std::time::Duration;

use crate::env::Env;

pub struct JwtConfig {
    pub secret: String,
    pub access_expiration: Duration,
    pub refresh_expiration: Duration,
}

impl JwtConfig {
    pub fn init(env: &Env) -> Self {
        let access_expiration = Duration::new(env.access_token_expires_mins * 60, 0);
        let refresh_expiration = Duration::new(env.refresh_token_expires_days * 24 * 60 * 60, 0);
        Self {
            secret: env.jwt_secret.clone(),
            access_expiration,
            refresh_expiration,
        }
    }
}
