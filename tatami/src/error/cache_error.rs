use axum::http::StatusCode;
use axum::Json;

use crate::error::utils::{reason, ErrorResponseBody, INTERNAL_REASON};

pub fn redis_error_to_response_tuple(
    err: &redis::RedisError,
) -> (StatusCode, Json<ErrorResponseBody>) {
    tracing::error!("redis error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
