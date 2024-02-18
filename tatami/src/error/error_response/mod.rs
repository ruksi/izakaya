use std::collections::HashMap;
use std::fmt::Display;

use axum::Json;

pub const REASON_INVALID: &str = "Validation failed";
pub const REASON_INTERNAL: &str = "Something went wrong";

pub fn error_message<T: Display>(message: T) -> Json<ErrorOut> {
    Json(ErrorOut::new(message.to_string()))
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ErrorOut {
    // user-friendly answer to the question: "Why did this error happen?"
    pub message: String,

    // optional `validator::ValidationErrors`-style errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<IssueMapOut>,
}

impl ErrorOut {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
            issues: None,
        }
    }

    pub fn with_issue_map(mut self, issues: IssueMapOut) -> Self {
        self.issues = Some(issues);
        self
    }

    pub fn with_validator_errors(mut self, errors: validator::ValidationErrors) -> Self {
        self.issues = Some(errors.into());
        self
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct IssueMapOut(HashMap<String, IssueKind>);

impl IssueMapOut {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_issue(mut self, field: String, new_issue: IssueOut) -> Self {
        let errors = self
            .0
            .entry(field)
            .or_insert_with(|| IssueKind::Field(vec![]));
        #[allow(unreachable_patterns)]
        match errors {
            IssueKind::Field(ref mut issues) => {
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
pub enum IssueKind {
    Field(Vec<IssueOut>),
}

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

impl From<validator::ValidationErrors> for IssueMapOut {
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
                            IssueOut {
                                code,
                                message,
                                params: params
                                    .iter()
                                    .map(|(k, v)| (k.to_string(), v.clone()))
                                    .collect(),
                            }
                        })
                        .collect();
                    (field_name.to_owned(), IssueKind::Field(errs))
                }
                _ => unimplemented!(), // implement nested variants when I need them
            })
            .collect();
        IssueMapOut(destination)
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
        let json = error_message("hello");
        assert_eq!(json.message, "hello");
        assert_eq!(json.issues, None);
    }

    #[tokio::test]
    async fn error_response_body_works() {
        let error = ErrorOut {
            message: "hello".into(),
            issues: None,
        };
        assert_eq!(error.message, "hello");
        assert_eq!(error.issues, None);
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
        let issues: IssueMapOut = err.into();
        issues.assert_json(json!({
            "first_name": [{
                "code": "length",
                "params": {"value": "bo", "min": 3}
            }]
        }));
    }
}
