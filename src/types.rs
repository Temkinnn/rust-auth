use sqlx::{Pool, Postgres};

use crate::errors::AppError;

pub type Database = Pool<Postgres>;
pub type Id = String;

pub type DatabaseResult<T> = Result<T, sqlx::Error>;
pub type AppResult<T> = Result<T, AppError>;
