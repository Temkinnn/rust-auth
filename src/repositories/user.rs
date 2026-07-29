use crate::{
    models::user::{CreateUserRepoDto, Role, UpdateUserDto, User},
    types::{Database, DatabaseResult, Id},
};

pub struct UserRepository(Database);

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self(db)
    }

    pub async fn create_user(&self, dto: CreateUserRepoDto) -> DatabaseResult<User> {
        sqlx::query_as!(
            User,
            "
            Insert into users (id, username, email, password, role)
            Values ($1, $2, $3, $4, $5)
            Returning id, username, email, password, role as \"role: Role\"
            ",
            dto.id,
            dto.username,
            dto.email,
            dto.password,
            dto.role as Option<Role>
        )
        .fetch_one(&self.0)
        .await
    }

    pub async fn get_users(&self, limit: i64, offset: i64) -> DatabaseResult<Vec<User>> {
        sqlx::query_as!(
            User,
            "
            Select id, username, email, password, role as \"role: Role\" from users
            Order by id
            Limit $1 offset $2
            ",
            limit,
            offset
        )
        .fetch_all(&self.0)
        .await
    }

    pub async fn get_user_by_id(&self, id: Id) -> DatabaseResult<Option<User>> {
        sqlx::query_as!(
            User,
            "
            Select id, username, email, password, role as \"role: Role\" from users
            Where id = $1
            ",
            id
        )
        .fetch_optional(&self.0)
        .await
    }

    pub async fn get_user_by_email(&self, email: &String) -> DatabaseResult<Option<User>> {
        sqlx::query_as!(
            User,
            "
            Select id, username, email, password, role as \"role: Role\" from users
            Where email = $1
            ",
            email
        )
        .fetch_optional(&self.0)
        .await
    }

    pub async fn update_user_by_id(
        &self,
        id: Id,
        dto: UpdateUserDto,
    ) -> DatabaseResult<Option<User>> {
        sqlx::query_as!(
            User,
            "
            Update users
            Set
                username = Coalesce($2, username),
                email = Coalesce($3, email),
                password = Coalesce($4, password),
                role = Coalesce($5, role)
            Where id = $1
            Returning id, username, email, password, role as \"role: Role\"
            ",
            id,
            dto.username,
            dto.email,
            dto.password,
            dto.role as Option<Role>
        )
        .fetch_optional(&self.0)
        .await
    }

    pub async fn delete_user_by_id(&self, id: Id) -> DatabaseResult<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            Delete from users
            Where id = $1
            Returning id, username, email, password, role as "role: Role"
            "#,
            id
        )
        .fetch_optional(&self.0)
        .await
    }
}
