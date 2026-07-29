use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Id,
    pub exp: usize,
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}
