use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{error_message, ErrorOut, INTERNAL_REASON};

pub fn argon2_password_hash_error_to_response_tuple(
    err: &argon2::password_hash::Error,
) -> (StatusCode, Json<ErrorOut>) {
    tracing::error!("argon2 password hash error: {:?}", err);
    let outbound = error_message(INTERNAL_REASON);
    (StatusCode::INTERNAL_SERVER_ERROR, outbound)
}
