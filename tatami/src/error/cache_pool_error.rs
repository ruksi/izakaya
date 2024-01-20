use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{json_message, ErrorBody, INTERNAL_REASON};

pub fn deadpool_redis_error_to_response_tuple(
    err: &deadpool::managed::PoolError<redis::RedisError>,
) -> (StatusCode, Json<ErrorBody>) {
    tracing::error!("redis pool error: {:?}", err);
    let error_body = json_message(INTERNAL_REASON);
    (StatusCode::INTERNAL_SERVER_ERROR, error_body)
}
