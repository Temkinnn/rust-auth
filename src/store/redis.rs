use crate::types::{AppResult, Redis};

pub struct RedisPool;

impl RedisPool {
    pub async fn init(url: &str) -> AppResult<Redis> {
        let client = redis::Client::open(url)?;
        let connection = client.get_multiplexed_async_connection().await?;
        Ok(connection)
    }
}
