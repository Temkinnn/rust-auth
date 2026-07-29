use crate::{
    repositories::{token::TokenRepository, user::UserRepository},
    types::{Database, Redis},
};

pub struct Repositories {
    pub token: TokenRepository,
    pub user: UserRepository,
}

impl Repositories {
    pub fn init(db: Database, redis: Redis) -> Self {
        Self {
            token: TokenRepository::new(redis),
            user: UserRepository::new(db),
        }
    }
}
