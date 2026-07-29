use crate::types::{AppResult, Redis};

pub struct RedisPool;

impl RedisPool {
    pub async fn init(url: &str) -> AppResult<Redis> {
        let client = redis::Client::open(url)?;
        Ok(client)
    }
}
