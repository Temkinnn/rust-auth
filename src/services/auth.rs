use std::{sync::Arc, time::Instant};

use tracing::instrument;

use crate::{
    errors::AppError,
    models::{
        auth::{LoginCredentials, RegistrationCredentials},
        token::Tokens,
        user::{CreateUserDto, Role},
    },
    services::{password::PasswordService, token::TokenService, user::UserService},
    types::AppResult,
};

pub struct AuthService {
    user_service: Arc<UserService>,
    token_service: Arc<TokenService>,
    password_service: PasswordService,
}

impl AuthService {
    pub fn new(user_service: Arc<UserService>, token_service: Arc<TokenService>) -> Self {
        Self {
            user_service,
            token_service,
            password_service: PasswordService::new(),
        }
    }

    pub async fn register(&self, data: RegistrationCredentials) -> AppResult<Tokens> {
        let user_exists = match self.user_service.get_user_by_email(&data.email).await {
            Ok(_) => true,
            Err(AppError::NotFound) => false,
            Err(e) => return Err(e),
        };

        if user_exists {
            return Err(AppError::AlreadyExists);
        }

        let hashed_password = self.password_service.hash_password(&data.password)?;

        let user = self
            .user_service
            .create_user(CreateUserDto {
                email: data.email,
                password: hashed_password,
                username: data.username,
                role: Some(Role::User),
            })
            .await?;

        let tokens = self.token_service.generate_tokens(user.id)?;
        self.token_service
            .save_refresh_token(tokens.refresh_token.clone())
            .await?;
        Ok(tokens)
    }

    pub async fn login(&self, data: LoginCredentials) -> AppResult<Tokens> {
        let user = self.user_service.get_user_by_email(&data.email).await?;

        let verified_password = self
            .password_service
            .verify_password(&data.password, &user.password)?;

        if !verified_password {
            return Err(AppError::InvalidCredentials);
        }

        let tokens = self.token_service.generate_tokens(user.id)?;
        self.token_service
            .save_refresh_token(tokens.refresh_token.clone())
            .await?;

        Ok(tokens)
    }

    pub async fn refresh(&self, refresh_token: String) -> AppResult<String> {
        let verified_token = self.token_service.verify_refresh_token(&refresh_token)?;

        let user = self.user_service.get_user_by_id(verified_token.sub).await?;

        let tokens = self.token_service.generate_tokens(user.id)?;
        self.token_service
            .update_refresh_token(tokens.refresh_token)
            .await?;

        Ok(tokens.access_token)
    }

    pub async fn logout(&self, refresh_token: String) -> AppResult<()> {
        let verified_token = self.token_service.verify_refresh_token(&refresh_token)?;
        self.token_service
            .delete_refresh_token(verified_token.jti)
            .await
    }
}
