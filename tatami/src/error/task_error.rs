use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::error::utils::{reason, INTERNAL_REASON};

pub fn tokio_task_join_error_into_response(
    err: &tokio::task::JoinError,
) -> (StatusCode, Json<Value>) {
    tracing::error!("tokio task join error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
