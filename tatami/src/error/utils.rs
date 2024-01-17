use std::fmt::Display;

use axum::Json;
use serde_json::{json, Value};

pub fn reason<T: Display>(reason: T) -> Json<Value> {
    Json(json!({
        "reason": reason.to_string(),
    }))
}
