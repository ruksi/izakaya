use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{json_message, ErrorBody};

pub fn axum_json_rejection_to_response_tuple(
    rejection: &axum::extract::rejection::JsonRejection,
) -> (StatusCode, Json<ErrorBody>) {
    tracing::error!("axum json rejection: {:?}", rejection);
    (rejection.status(), json_message(rejection.body_text()))
}
