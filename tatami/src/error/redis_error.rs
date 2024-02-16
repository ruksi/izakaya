use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{error_message, ErrorOut, INTERNAL_REASON};

pub fn redis_error_to_response_tuple(err: &redis::RedisError) -> (StatusCode, Json<ErrorOut>) {
    tracing::error!("redis error: {:?}", err);
    let outbound = error_message(INTERNAL_REASON);
    (StatusCode::INTERNAL_SERVER_ERROR, outbound)
}
