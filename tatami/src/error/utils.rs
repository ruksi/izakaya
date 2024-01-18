use std::fmt::Display;

use axum::Json;
use serde_json::{json, Value};

// a generic error message when we don't want to expose what went wrong
pub const INTERNAL_REASON: &str = "something went wrong";

pub fn reason<T: Display>(reason: T) -> Json<Value> {
    Json(json!({
        "reason": reason.to_string(),
    }))
}
