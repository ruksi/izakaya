use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{json_message, ErrorBody};

mod cache_error;
mod cache_pool_error;
mod database_error;
mod error_response;
mod password_error;
mod task_error;
mod validation_error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Unauthorized,
    Forbidden,
    NotFound,
    JsonRejection(axum::extract::rejection::JsonRejection),
    Validation(validator::ValidationErrors),
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
            Unauthorized => (
                StatusCode::UNAUTHORIZED,
                json_message("Authentication required"),
            ),
            Forbidden => (
                StatusCode::FORBIDDEN,
                json_message("You cannot do this thing"),
            ),
            NotFound => (
                StatusCode::NOT_FOUND,
                json_message("The thing doesn't exist"),
            ),
            JsonRejection(err) => (err.status(), json_message(err.body_text())),
            Validation(err) => validation_error::validation_error_to_response_tuple(err),
            Database(err) => database_error::sqlx_error_to_response_tuple(err),
            Cache(err) => cache_error::redis_error_to_response_tuple(err),
            CachePool(err) => cache_pool_error::deadpool_redis_error_to_response_tuple(err),
            Password(err) => password_error::argon2_password_hash_error_to_response_tuple(err),
            Task(err) => task_error::tokio_task_join_error_to_response_tuple(err),
        }
    }

    #[cfg(test)]
    pub fn assert_status(&self, expected: StatusCode) -> &Self {
        assert_eq!(self.response_tuple().0, expected);
        self
    }

    #[cfg(test)]
    pub fn assert_json(&self, expected: serde_json::Value) -> &Self {
        let actual = serde_json::to_value(&self.response_tuple().1 .0).unwrap();
        assert_eq!(actual, expected);
        self
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.response_tuple().into_response()
    }
}

impl From<axum::extract::rejection::JsonRejection> for Error {
    fn from(err: axum::extract::rejection::JsonRejection) -> Self {
        Self::JsonRejection(err)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::Validation(err)
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
