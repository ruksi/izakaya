use axum::http::StatusCode;
use axum::Json;

use crate::error::error_response::{ErrorBody, INVALID_REASON};

pub fn validator_error_to_response_tuple(
    err: &validator::ValidationErrors,
) -> (StatusCode, Json<ErrorBody>) {
    let error_body = ErrorBody::new(INVALID_REASON).with_validator_details(err.clone());
    (StatusCode::BAD_REQUEST, Json(error_body))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;
    use validator::Validate;

    use crate::prelude::*;

    #[test]
    fn validator_works_as_expected() -> Result<()> {
        #[derive(Deserialize, Validate)]
        struct Person {
            #[validate(length(min = 3))]
            first_name: String,
        }
        let person = Person {
            first_name: "bo".into(),
        };
        let err = person.validate().unwrap_err();
        let text = serde_json::to_string(&err).unwrap();
        let untyped = serde_json::from_str::<serde_json::Value>(&text).unwrap();
        assert_eq!(
            untyped,
            json!({
                "first_name": [{
                    "code": "length",
                    "message": null, // we actually drop nulls in our version
                    "params": {
                        "min": 3,
                        "value": "bo",
                    },
                }],
            })
        );
        Ok(())
    }
}
