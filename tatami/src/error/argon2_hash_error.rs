use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{json_message, ErrorBody, INTERNAL_REASON};

pub fn argon2_password_hash_error_to_response_tuple(
    err: &argon2::password_hash::Error,
) -> (StatusCode, Json<ErrorBody>) {
    tracing::error!("argon2 password hash error: {:?}", err);
    let error_body = json_message(INTERNAL_REASON);
    (StatusCode::INTERNAL_SERVER_ERROR, error_body)
}
