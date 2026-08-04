use uuid::Uuid;

use crate::{
    errors::AppError,
    models::user::{CreateUserDto, CreateUserRepoDto, Role, UpdateUserDto, User},
    repositories::user::UserRepository,
    types::{AppResult, Id},
};

pub struct UserService(UserRepository);

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self(repo)
    }

    pub async fn create_user(&self, data: CreateUserDto) -> AppResult<User> {
        let id = Uuid::now_v7();
        Ok(self
            .0
            .create_user(CreateUserRepoDto {
                id,
                email: data.email,
                password: data.password,
                role: data.role,
                username: data.username,
            })
            .await?)
    }
    pub async fn get_users(&self, limit: Option<i64>, offset: Option<i64>) -> AppResult<Vec<User>> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);
        Ok(self.0.get_users(limit, offset).await?)
    }
    pub async fn get_user_by_id(&self, id: Id) -> AppResult<User> {
        self.0.get_user_by_id(id).await?.ok_or(AppError::NotFound)
    }
    pub async fn get_user_by_email(&self, email: &String) -> AppResult<User> {
        self.0
            .get_user_by_email(email)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn get_user_role_by_id(&self, id: Id) -> AppResult<Role> {
        let user_role = self
            .0
            .get_user_role_by_id(id)
            .await?
            .ok_or(AppError::NotFound)?;
        let role = user_role.role;
        Ok(role)
    }

    pub async fn update_user(&self, id: Id, data: UpdateUserDto) -> AppResult<User> {
        self.0
            .update_user_by_id(id, data)
            .await?
            .ok_or(AppError::NotFound)
    }
    pub async fn delete_user(&self, id: Id) -> AppResult<User> {
        self.0
            .delete_user_by_id(id)
            .await?
            .ok_or(AppError::NotFound)
    }
}
