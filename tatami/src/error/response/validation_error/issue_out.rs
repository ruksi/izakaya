use serde::Serialize;

use std::collections::HashMap;

/// Issue is a single validation error.

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct IssueOut {
    // what validation error happened e.g. `length`
    pub code: String,

    // a custom, human readable validation error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    // further details about the validation error e.g.
    // - `value` what was the _value_ that was checked (name is in the issue map)
    // - `min` what is the minimum length requirement
    pub details: HashMap<String, serde_json::Value>,
}

impl IssueOut {
    pub fn new<T: Into<String>>(code: T) -> Self {
        Self {
            code: code.into(),
            message: None,
            details: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_detail(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        let as_json_value = serde_json::to_value(value).unwrap();
        self.details.insert(key.into(), as_json_value);
        self
    }
}
