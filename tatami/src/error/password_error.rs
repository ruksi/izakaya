use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::error::utils::{reason, INTERNAL_REASON};

pub fn argon2_password_hash_error_into_response(
    err: &argon2::password_hash::Error,
) -> (StatusCode, Json<Value>) {
    tracing::error!("argon2 password hash error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
