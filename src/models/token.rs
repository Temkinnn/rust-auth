use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: Id,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: Id,
    pub jti: Id,
    pub exp: usize,
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}
