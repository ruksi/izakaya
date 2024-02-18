use serde::Serialize;

use std::collections::HashMap;

/// Issue is a single validation error.

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct IssueOut {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub params: HashMap<String, serde_json::Value>,
}

impl IssueOut {
    pub fn new<T: Into<String>>(code: T) -> Self {
        Self {
            code: code.into(),
            message: None,
            params: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_param(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        let as_json_value = serde_json::to_value(value).unwrap();
        self.params.insert(key.into(), as_json_value);
        self
    }
}
