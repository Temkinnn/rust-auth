use std::env;

pub struct Env {
    pub host: String,
    pub port: u16,

    pub database: String,
    pub redis: String,

    pub jwt_secret: String,
    pub access_token_expires_mins: u64,
    pub refresh_token_expires_days: u64,
}

impl Env {
    pub async fn init() -> Self {
        dotenvy::dotenv().expect("Failed to load enviroment variables");

        let host = env::var("HOST").expect("Failed to load 'host' variable");

        let port = env::var("PORT")
            .expect("Failed to load 'port' variable")
            .parse()
            .expect("Failed to parse 'port' variable");

        let database = env::var("DATABASE_URL").expect("Failed to load database url");
        let redis = env::var("REDIS_URL").expect("Failed to load redis url");

        let jwt_secret = env::var("JWT_SECRET").expect("Failed to load 'JWT_SECRET' variable");

        let access_token_expires_mins = env::var("ACCESS_TOKEN_EXPIRE_MINUTES")
            .expect("Failed to load 'ACCESS_TOKEN_EXPIRE_MINUTES' variable")
            .parse()
            .expect("Failed to parse 'ACCESS_TOKEN_EXPIRE_MINUTES' variable");

        let refresh_token_expires_days = env::var("REFRESH_TOKEN_EXPIRE_DAYS")
            .expect("Failed to load 'REFRESH_TOKEN_EXPIRE_DAYS' variable")
            .parse()
            .expect("Failed to parse 'REFRESH_TOKEN_EXPIRE_DAYS' variable");

        Env {
            host,
            port,
            database,
            redis,
            jwt_secret,
            access_token_expires_mins,
            refresh_token_expires_days,
        }
    }
}
