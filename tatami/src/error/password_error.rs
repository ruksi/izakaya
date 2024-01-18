use axum::http::StatusCode;
use axum::Json;

use crate::error::utils::{reason, ErrorBody, INTERNAL_REASON};

pub fn argon2_password_hash_error_to_response_tuple(
    err: &argon2::password_hash::Error,
) -> (StatusCode, Json<ErrorBody>) {
    tracing::error!("argon2 password hash error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
