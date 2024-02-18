use crate::error::response::IssueMapOut;

// All "controlled" error responses from the API should be in this format.

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ErrorOut {
    // user-friendly answer to the question: "Why did this error happen?"
    pub message: String,

    // optional `validator::ValidationErrors`-style errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<IssueMapOut>,
}

impl ErrorOut {
    pub fn new(message: impl Into<String>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn error_response_body_works() {
        let error = ErrorOut::new("Hello World!");
        assert_eq!(error.message, "Hello World!");
        assert_eq!(error.issues, None);
    }
}
