use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{error_message, ErrorOut, INTERNAL_REASON};

pub fn tokio_task_join_error_to_response_tuple(
    err: &tokio::task::JoinError,
) -> (StatusCode, Json<ErrorOut>) {
    tracing::error!("tokio task join error: {:?}", err);
    let outbound = error_message(INTERNAL_REASON);
    (StatusCode::INTERNAL_SERVER_ERROR, outbound)
}
