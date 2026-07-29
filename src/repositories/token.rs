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

    pub async fn save_token(&mut self, jti: Id, token: String, exp: u64) -> AppResult<()> {
        Ok(self.0.set_ex(format!("token:{jti}"), token, exp).await?)
    }

    pub async fn update_token(&mut self, jti: Id, token: String, exp: u64) -> AppResult<()> {
        Ok(self.0.set_ex(format!("token:{jti}"), token, exp).await?)
    }
    
    pub async fn delete_token(&mut self, jti: Id) -> AppResult<()> {
        let num = self.0.del(format!("token:{jti}")).await?;

        if num == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(())
        }
    }

}
