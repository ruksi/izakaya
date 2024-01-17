use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::error::utils::reason;

pub fn argon2_password_hash_error_into_response(
    err: &argon2::password_hash::Error,
) -> (StatusCode, Json<Value>) {
    (StatusCode::INTERNAL_SERVER_ERROR, reason(err))
}
