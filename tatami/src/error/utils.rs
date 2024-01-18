use std::fmt::Display;

use axum::Json;

// a generic error message for when we don't want to expose what went wrong,
// you should always log the full details before returning this reason
pub const INTERNAL_REASON: &str = "something went wrong";

#[derive(serde::Serialize)]
pub struct ErrorBody {
    pub reason: String,
    // pub detail: Option<Json<serde_json::Value>>, // details like the value that caused the error
    // pub hint: Option<String>, // how to fix the problem (if applicable)
}

pub fn reason<T: Display>(reason: T) -> Json<ErrorBody> {
    Json(ErrorBody {
        reason: reason.to_string(),
    })
}
