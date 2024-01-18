use axum::http::StatusCode;
use axum::Json;

use crate::error::utils::{reason, ErrorResponseBody, INTERNAL_REASON};

pub fn tokio_task_join_error_to_response_tuple(
    err: &tokio::task::JoinError,
) -> (StatusCode, Json<ErrorResponseBody>) {
    tracing::error!("tokio task join error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
