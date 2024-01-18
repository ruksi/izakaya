use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::error::utils::{reason, INTERNAL_REASON};

pub fn deadpool_redis_error_into_response(
    err: &deadpool::managed::PoolError<redis::RedisError>,
) -> (StatusCode, Json<Value>) {
    tracing::error!("redis pool error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
