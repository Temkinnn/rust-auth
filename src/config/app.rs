use crate::{
    env::Env,
    store::{db::DatabasePool, redis::RedisPool},
    types::{Database, Redis},
};

pub struct AppConfig {
    pub env: Env,
    pub db: Database,
    pub cache: Redis,
}

impl AppConfig {
    pub async fn init() -> Self {
        tracing_subscriber::fmt::init();

        let env = Self::env_init().await;
        let db = Self::db_init(&env.database).await;
        let cache = Self::cache_init(&env.redis).await;

        Self { env, db, cache }
    }

    async fn db_init(url: &str) -> Database {
        DatabasePool::init(url)
            .await
            .expect("Failed to load Database pool")
    }

    async fn cache_init(url: &str) -> Redis {
        RedisPool::init(url)
            .await
            .expect("Failed to load Redis pool")
    }

    pub async fn env_init() -> Env {
        Env::init().await
    }
}
