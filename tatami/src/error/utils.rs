use std::fmt::Display;

use axum::Json;

// a generic error message for when we don't want to expose what went wrong,
// you should always log the full details before returning this reason
pub const INTERNAL_REASON: &str = "Something went wrong";

#[derive(serde::Serialize, Debug)]
pub struct ErrorResponseBody {
    // user-friendly answer to the question: "Why did this error happen?"
    pub reason: String,

    // optional further information about the error, possibly shown depending on context, e.g.:
    //   - name: human-readable, capitalized name of the _thing_ that caused the error
    //   - value: human-readable value of the _thing_ that caused the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,

    // optional, user-friendly answer to the question: "What can I do to fix this?"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

pub fn reason<T: Display>(reason: T) -> Json<ErrorResponseBody> {
    Json(ErrorResponseBody {
        reason: reason.to_string(),
        detail: None,
        hint: None,
    })
}
