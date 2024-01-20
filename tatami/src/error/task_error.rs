use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{json_message, ErrorBody, INTERNAL_REASON};

pub fn tokio_task_join_error_to_response_tuple(
    err: &tokio::task::JoinError,
) -> (StatusCode, Json<ErrorBody>) {
    tracing::error!("tokio task join error: {:?}", err);
    let error_body = json_message(INTERNAL_REASON);
    (StatusCode::INTERNAL_SERVER_ERROR, error_body)
}
