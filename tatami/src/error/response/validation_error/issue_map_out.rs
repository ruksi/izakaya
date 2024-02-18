use std::collections::HashMap;

use super::issue_out::IssueOut;

// Issue Map is a collection of validation errors resulting from a request.

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum IssueOutKind {
    Field(Vec<IssueOut>), // validation errors (issues) related to a specific field
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct IssueMapOut(HashMap<String, IssueOutKind>);

impl IssueMapOut {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_field_issue(mut self, field: String, new_issue: IssueOut) -> Self {
        let field_issues = self
            .0
            .entry(field)
            .or_insert_with(|| IssueOutKind::Field(vec![]));
        match field_issues {
            IssueOutKind::Field(ref mut inner) => inner.push(new_issue),
        }
        self
    }

    #[cfg(test)]
    pub fn assert_json(&self, expected: serde_json::Value) -> &Self {
        let actual = serde_json::to_value(self).unwrap();
        assert_eq!(actual, expected);
        self
    }

    #[cfg(test)]
    pub fn assert_field_code(&self, field: &str, code: &str) -> &Self {
        let issues = self.0.get(field).unwrap();
        match issues {
            IssueOutKind::Field(issues) => {
                let issue = issues.iter().find(|issue| issue.code == code).unwrap();
                assert_eq!(issue.code, code);
            }
        }
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
                    (field_name.to_owned(), IssueOutKind::Field(errs))
                }
                _ => unimplemented!(), // implement nested variants when I need them
            })
            .collect();
        IssueMapOut(destination)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::response::IssueMapOut;
    use serde::Deserialize;
    use serde_json::json;
    use validator::Validate;

    #[tokio::test]
    async fn error_response_from_validator_errors_works() {
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

        // full issue check
        issues.assert_json(json!({
            "first_name": [{
                "code": "length",
                "params": {"value": "bo", "min": 3}
            }]
        }));

        // "field 'first_name' has validation errors related to 'length'"
        issues.assert_field_code("first_name", "length");
    }
}
