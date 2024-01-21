use std::collections::HashMap;
use std::fmt::Display;

use axum::Json;

// a generic error message for when we don't want to expose what went wrong,
// you should always log the full details before returning this reason
pub const INVALID_REASON: &str = "Validation failed";
pub const INTERNAL_REASON: &str = "Something went wrong";

pub fn json_message<T: Display>(message: T) -> Json<ErrorBody> {
    Json(ErrorBody::new(message.to_string()))
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ErrorBody {
    // user-friendly answer to the question: "Why did this error happen?"
    pub message: String,

    // optional `validator::ValidationErrors`-style errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
}

impl ErrorBody {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
            details: None,
        }
    }

    pub fn with_error_details(mut self, details: ErrorDetails) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_validator_details(mut self, details: validator::ValidationErrors) -> Self {
        self.details = Some(details.into());
        self
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ErrorDetails(HashMap<String, ErrorKind>);

impl ErrorDetails {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_issue(mut self, field: String, new_issue: Issue) -> Self {
        let errors = self
            .0
            .entry(field)
            .or_insert_with(|| ErrorKind::Field(vec![]));
        #[allow(unreachable_patterns)]
        match errors {
            ErrorKind::Field(ref mut issues) => {
                issues.push(new_issue);
            }
            _ => unimplemented!(),
        }
        self
    }

    #[cfg(test)]
    pub fn assert_json(&self, expected: serde_json::Value) -> &Self {
        let actual = serde_json::to_value(self).unwrap();
        assert_eq!(actual, expected);
        self
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ErrorKind {
    Field(Vec<Issue>),
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub params: HashMap<String, serde_json::Value>,
}

impl Issue {
    pub fn new<T: Into<String>>(code: T) -> Self {
        Self {
            code: code.into(),
            message: None,
            params: HashMap::new(),
        }
    }

    pub fn with_message<T: Into<String>>(mut self, message: T) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_param<T: Into<String>, V: serde::Serialize>(mut self, key: T, value: V) -> Self {
        let as_json_value = serde_json::to_value(value).unwrap();
        self.params.insert(key.into(), as_json_value);
        self
    }
}

impl From<validator::ValidationErrors> for ErrorDetails {
    fn from(err: validator::ValidationErrors) -> Self {
        let source = err.into_errors();
        // the 'statics are killing my groove with this one,
        // I need it to be more dynamic for my use-case
        let destination = source
            .into_iter()
            .map(|(field_name, validation_err)| match validation_err {
                validator::ValidationErrorsKind::Field(mut errs) => {
                    let errs = errs
                        .drain(..)
                        .map(|err| {
                            let code = err.code.to_string();
                            let message = err.message.map(|s| s.to_string());
                            let params = err.params;
                            Issue {
                                code,
                                message,
                                params: params
                                    .iter()
                                    .map(|(k, v)| (k.to_string(), v.clone()))
                                    .collect(),
                            }
                        })
                        .collect();
                    (field_name.to_owned(), ErrorKind::Field(errs))
                }
                _ => unimplemented!(), // implement nested variants when I need them
            })
            .collect();
        ErrorDetails(destination)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;
    use validator::Validate;

    use super::*;

    #[tokio::test]
    async fn json_message_works() {
        let json = json_message("hello");
        assert_eq!(json.message, "hello");
        assert_eq!(json.details, None);
    }

    #[tokio::test]
    async fn error_response_body_works() {
        let body = ErrorBody {
            message: "hello".into(),
            details: None,
        };
        assert_eq!(body.message, "hello");
        assert_eq!(body.details, None);
    }

    #[tokio::test]
    async fn error_response_body_from_validation_errors_works() {
        #[derive(Deserialize, Validate)]
        struct Person {
            #[validate(length(min = 3))]
            first_name: String,
        }
        let person = Person {
            first_name: "bo".into(),
        };
        let err = person.validate().unwrap_err();
        let details: ErrorDetails = err.into();
        details.assert_json(json!({
            "first_name": [{
                "code": "length",
                "params": {"value": "bo", "min": 3}
            }]
        }));
    }
}