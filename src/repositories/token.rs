use redis::AsyncTypedCommands;

use crate::{
    errors::AppError,
    types::{AppResult, Id, Redis},
};

pub struct TokenRepository(Redis);

impl TokenRepository {
    pub fn new(pool: Redis) -> Self {
        Self(pool)
    }

    pub async fn save_token(&self, jti: Id, token: String, exp: u64) -> AppResult<()> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        Ok(conn.set_ex(format!("token:{jti}"), token, exp).await?)
    }

    pub async fn update_token(&self, jti: Id, token: String, exp: u64) -> AppResult<()> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        Ok(conn.set_ex(format!("token:{jti}"), token, exp).await?)
    }

    pub async fn delete_token(&self, jti: Id) -> AppResult<()> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let num = conn.del(format!("token:{jti}")).await?;

        if num == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(())
        }
    }
}
