use axum::http::StatusCode;
use axum::Json;

use crate::error::utils::{reason, ErrorBody, INTERNAL_REASON};

pub fn deadpool_redis_error_to_response_tuple(
    err: &deadpool::managed::PoolError<redis::RedisError>,
) -> (StatusCode, Json<ErrorBody>) {
    tracing::error!("redis pool error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
