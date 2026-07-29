use std::sync::Arc;

use crate::{
    config::{jwt::JwtConfig, repositories::Repositories},
    services::{
        auth::AuthService, password::PasswordService, token::TokenService, user::UserService,
    },
};

pub struct Services {
    pub auth: AuthService,
    pub user: Arc<UserService>,
    pub token: Arc<TokenService>,
    pub password: PasswordService,
}

impl Services {
    pub fn init(repositories: Repositories, jwt: JwtConfig) -> Self {
        let user_service = Arc::new(UserService::new(repositories.user));
        let token_service = Arc::new(TokenService::new(jwt, repositories.token));

        Self {
            auth: AuthService::new(user_service.clone(), token_service.clone()),
            password: PasswordService::new(),
            user: user_service,
            token: token_service,
        }
    }
}
