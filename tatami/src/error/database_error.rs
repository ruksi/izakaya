use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::error::utils::{reason, INTERNAL_REASON};

pub fn sqlx_error_into_response(err: &sqlx::Error) -> (StatusCode, Json<Value>) {
    // check if the error code is our custom SQLSTATE error code
    // if it is, this is a safe, known error to expose to the client
    if let sqlx::Error::Database(err) = err {
        if let Some(sqlstate_code) = err.code() {
            if sqlstate_code == "WRONG" {
                return (StatusCode::BAD_REQUEST, reason(err.message()));
            }
        };
    };

    tracing::error!("sqlx error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
