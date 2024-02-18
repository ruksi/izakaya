use axum::http::StatusCode;
use axum::Json;

use crate::error::response::{error_message, ErrorOut};

pub fn axum_json_rejection_to_response_tuple(
    rejection: &axum::extract::rejection::JsonRejection,
) -> (StatusCode, Json<ErrorOut>) {
    tracing::error!("axum json rejection: {:?}", rejection);
    let outbound = error_message(rejection.body_text());
    (rejection.status(), outbound)
}
