use axum::http::StatusCode;
use axum::Json;

use crate::error::response::{error_message, ErrorOut, REASON_INTERNAL};

pub fn password_hash_error_to_response_tuple(
    err: &argon2::password_hash::Error,
) -> (StatusCode, Json<ErrorOut>) {
    tracing::error!("password hash error: {:?}", err);
    let outbound = error_message(REASON_INTERNAL);
    (StatusCode::INTERNAL_SERVER_ERROR, outbound)
}
