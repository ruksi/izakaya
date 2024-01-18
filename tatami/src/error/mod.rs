use axum::http::StatusCode;
use axum::Json;

use crate::error::utils::{reason, ErrorBody};

mod cache_error;
mod cache_pool_error;
mod database_error;
mod password_error;
mod task_error;
mod utils;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Unauthorized,
    Forbidden,
    NotFound,
    Database(sqlx::Error),
    Cache(redis::RedisError),
    CachePool(deadpool::managed::PoolError<redis::RedisError>),
    Password(argon2::password_hash::Error),
    Task(tokio::task::JoinError),
}

impl Error {
    pub fn response_tuple(&self) -> (StatusCode, Json<ErrorBody>) {
        use Error::*;
        match self {
            Unauthorized => (StatusCode::UNAUTHORIZED, reason("authentication required")),
            Forbidden => (StatusCode::FORBIDDEN, reason("you cannot do this thing")),
            NotFound => (StatusCode::NOT_FOUND, reason("the thing doesn't exist")),
            Database(err) => database_error::sqlx_error_to_response_tuple(err),
            Cache(err) => cache_error::redis_error_to_response_tuple(err),
            CachePool(err) => cache_pool_error::deadpool_redis_error_to_response_tuple(err),
            Password(err) => password_error::argon2_password_hash_error_to_response_tuple(err),
            Task(err) => task_error::tokio_task_join_error_to_response_tuple(err),
        }
    }

    #[cfg(test)]
    pub fn status(&self) -> StatusCode {
        self.response_tuple().0
    }

    #[cfg(test)]
    pub fn reason(&self) -> String {
        self.response_tuple().1.reason.clone()
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.response_tuple().into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Self::Cache(err)
    }
}

impl From<deadpool::managed::PoolError<redis::RedisError>> for Error {
    fn from(err: deadpool::managed::PoolError<redis::RedisError>) -> Self {
        Self::CachePool(err)
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::Password(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::Task(err)
    }
}
