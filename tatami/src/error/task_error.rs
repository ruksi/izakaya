use crate::error::utils::reason;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

pub fn tokio_task_join_error_into_response(
    err: &tokio::task::JoinError,
) -> (StatusCode, Json<Value>) {
    (StatusCode::INTERNAL_SERVER_ERROR, reason(err))
}
