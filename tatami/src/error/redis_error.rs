use axum::http::StatusCode;
use axum::Json;

use crate::error::response::{error_message, ErrorOut, REASON_INTERNAL};

pub fn redis_error_to_response_tuple(err: &redis::RedisError) -> (StatusCode, Json<ErrorOut>) {
    tracing::error!("redis error: {:?}", err);
    let outbound = error_message(REASON_INTERNAL);
    (StatusCode::INTERNAL_SERVER_ERROR, outbound)
}
