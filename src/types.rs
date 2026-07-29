use redis::aio::MultiplexedConnection;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::errors::AppError;

pub type Database = Pool<Postgres>;
pub type Redis = MultiplexedConnection;
pub type Id = Uuid;

pub type DatabaseResult<T> = Result<T, sqlx::Error>;
pub type AppResult<T> = Result<T, AppError>;
