use sqlx::PgPool;

use crate::types::{AppResult, Database};

pub struct DatabasePool;

impl DatabasePool {
    pub async fn init(url: &str) -> AppResult<Database> {
        let pool = PgPool::connect(url).await?;

        let _ = sqlx::migrate!().run(&pool).await;
        Ok(pool)
    }
}
