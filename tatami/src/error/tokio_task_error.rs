use axum::http::StatusCode;
use axum::Json;

use crate::error::response::{error_message, ErrorOut, REASON_INTERNAL};

pub fn tokio_task_join_error_to_response_tuple(
    err: &tokio::task::JoinError,
) -> (StatusCode, Json<ErrorOut>) {
    tracing::error!("Tokio task join error: {:?}", err);
    let outbound = error_message(REASON_INTERNAL);
    (StatusCode::INTERNAL_SERVER_ERROR, outbound)
}
