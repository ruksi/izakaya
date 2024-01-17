use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::error::utils::reason;

pub fn deadpool_redis_error_into_response(
    err: &deadpool::managed::PoolError<redis::RedisError>,
) -> (StatusCode, Json<Value>) {
    (StatusCode::INTERNAL_SERVER_ERROR, reason(err))
}
