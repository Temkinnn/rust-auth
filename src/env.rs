use std::env;

pub struct Env {
    pub host: String,
    pub port: u16,
    pub database: String,

    pub jwt_secret: String,
    pub access_token_expires_month: u16,
    pub refresh_token_expires_days: u16,
}

impl Env {
    pub fn init() -> Self {
        dotenvy::dotenv().expect("Failed to load enviroment variables");

        let host = env::var("HOST").expect("Failed to load 'host' variable");

        let port = env::var("PORT")
            .expect("Failed to load 'port' variable")
            .parse()
            .expect("Failed to parse 'port' variable");

        let database = env::var("DATABASE_URL").expect("Failed to load database url");

        let jwt_secret = env::var("JWT_SECRET").expect("Failed to load database url");

        let access_token_expires_month = env::var("ACCESS_TOKEN_EXPIRE_MINUTES")
            .expect("Failed to load 'port' variable")
            .parse()
            .expect("Failed to parse 'port' variable");

        let refresh_token_expires_days = env::var("REFRESH_TOKEN_EXPIRE_DAYS")
            .expect("Failed to load 'port' variable")
            .parse()
            .expect("Failed to parse 'port' variable");

        Env {
            host,
            port,
            database,
            jwt_secret,
            access_token_expires_month,
            refresh_token_expires_days,
        }
    }
}
