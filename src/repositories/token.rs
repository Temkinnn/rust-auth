use redis::AsyncTypedCommands;

use crate::{
    errors::AppError,
    types::{AppResult, Id, Redis, Token},
};

pub struct TokenRepository(Redis);

impl TokenRepository {
    pub fn new(pool: Redis) -> Self {
        Self(pool)
    }

    pub async fn get_token(&self, jti: Id) -> AppResult<Option<Token>> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        Ok(conn.get(format!("token:{jti}")).await?)
    }

    pub async fn save_token(&self, jti: Id, token: Token, exp: u64) -> AppResult<()> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        Ok(conn.set_ex(format!("token:{jti}"), token, exp).await?)
    }

    pub async fn update_token(
        &self,
        jti: Id,
        new_jti: Id,
        token: Token,
        exp: u64,
    ) -> AppResult<()> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let num = conn.del(format!("token:{jti}")).await?;

        if num == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(conn.set_ex(format!("token:{new_jti}"), token, exp).await?)
        }
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
