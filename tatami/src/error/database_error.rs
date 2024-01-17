use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::error::utils::reason;

pub fn sqlx_error_into_response(err: &sqlx::Error) -> (StatusCode, Json<Value>) {
    // TODO: should we obfuscate the error messages in production?

    let sqlx::Error::Database(err) = err else {
        return (StatusCode::INTERNAL_SERVER_ERROR, reason(err));
    };

    // check if the error code is our custom SQLSTATE error code
    // if it is, we know this error is safe to expose to the client
    let Some(sqlstate_code) = err.code() else {
        return (StatusCode::INTERNAL_SERVER_ERROR, reason(err));
    };
    if sqlstate_code != "WRONG" {
        return (StatusCode::INTERNAL_SERVER_ERROR, reason(err));
    }

    (StatusCode::BAD_REQUEST, reason(err.message()))
}
