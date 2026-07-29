use std::time::Duration;



pub struct JwtConfig {
    pub secret: String,
    pub access_expiration: Duration,
    pub refresh_expiration: Duration,
}

