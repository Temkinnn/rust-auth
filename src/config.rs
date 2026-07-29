use crate::{
    env::Env,
    store::{db::DatabasePool, redis::RedisPool},
    types::{Database, Redis},
};

pub struct AppConfig {
    pub env: Env,
    pub db: Database,
    pub redis: Redis,
}

impl AppConfig {
    pub async fn init() -> Self {
        tracing_subscriber::fmt::init();

        let env = Env::init();
        let db = DatabasePool::init(&env.database)
            .await
            .expect("Failed to load Database pool");
        
        let redis = RedisPool::init(&env.redis)
            .await
            .expect("Failed to load Redis pool");

        Self { env, db, redis }
    }
}
