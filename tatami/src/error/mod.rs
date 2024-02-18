use axum::http::StatusCode;
use axum::Json;

use crate::error::argon2_hash_error::argon2_password_hash_error_to_response_tuple;
use crate::error::axum_json_rejection::axum_json_rejection_to_response_tuple;
use crate::error::redis_error::redis_error_to_response_tuple;
use crate::error::redis_pool_error::deadpool_redis_error_to_response_tuple;
use crate::error::response::{error_message, ErrorOut};
use crate::error::sqlx_error::sqlx_error_to_response_tuple;
use crate::error::tokio_task_error::tokio_task_join_error_to_response_tuple;
use crate::error::validation_error::validator_error_to_response_tuple;

mod argon2_hash_error;
mod axum_json_rejection;
mod redis_error;
mod redis_pool_error;
mod response;
mod sqlx_error;
mod tokio_task_error;
mod validation_error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    AxumJsonRejection(axum::extract::rejection::JsonRejection),
    Validator(validator::ValidationErrors),
    Sqlx(sqlx::Error),
    Redis(redis::RedisError),
    RedisPool(deadpool::managed::PoolError<redis::RedisError>),
    Argon2Hash(argon2::password_hash::Error),
    TokioTask(tokio::task::JoinError),
}

impl Error {
    pub fn response_tuple(&self) -> (StatusCode, Json<ErrorOut>) {
        use Error::*;
        match self {
            BadRequest => (
                StatusCode::BAD_REQUEST,
                error_message("Bad request, check your parameters"),
            ),
            Unauthorized => (
                StatusCode::UNAUTHORIZED,
                error_message("Authentication required"),
            ),
            Forbidden => (
                StatusCode::FORBIDDEN,
                error_message("You cannot do this thing"),
            ),
            NotFound => (
                StatusCode::NOT_FOUND,
                error_message("The thing doesn't exist"),
            ),
            AxumJsonRejection(rejection) => axum_json_rejection_to_response_tuple(rejection),
            Validator(err) => validator_error_to_response_tuple(err),
            Sqlx(err) => sqlx_error_to_response_tuple(err),
            Redis(err) => redis_error_to_response_tuple(err),
            RedisPool(err) => deadpool_redis_error_to_response_tuple(err),
            Argon2Hash(err) => argon2_password_hash_error_to_response_tuple(err),
            TokioTask(err) => tokio_task_join_error_to_response_tuple(err),
        }
    }

    #[cfg(test)]
    pub fn assert_status(&self, expected: StatusCode) -> &Self {
        assert_eq!(self.response_tuple().0, expected);
        self
    }

    #[cfg(test)]
    pub fn assert_json(&self, expected: serde_json::Value) -> &Self {
        let actual = serde_json::to_value(self.response_tuple().1 .0).unwrap();
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
        Self::AxumJsonRejection(err)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::Validator(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Self::Redis(err)
    }
}

impl From<deadpool::managed::PoolError<redis::RedisError>> for Error {
    fn from(err: deadpool::managed::PoolError<redis::RedisError>) -> Self {
        Self::RedisPool(err)
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::Argon2Hash(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::TokioTask(err)
    }
}
