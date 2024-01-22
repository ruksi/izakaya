use axum::http::StatusCode;
use axum::Json;
use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::postgres::PgDatabaseError;

use crate::error::error_response::{
    json_message, ErrorBody, ErrorDetails, Issue, INTERNAL_REASON, INVALID_REASON,
};

// https://www.postgresql.org/docs/current/errcodes-appendix.html
pub const UNIQUE_VIOLATION: &str = "23505";

pub fn sqlx_error_to_response_tuple<'a>(err: &'a sqlx::Error) -> (StatusCode, Json<ErrorBody>) {
    // check if the error code is our custom SQLSTATE error code
    // if it is, this is a safe, known error to expose to the client
    // if let sqlx::Error::Database(err) = err {
    //     if let Some(sqlstate_code) = err.code() {
    //         if sqlstate_code == "WRONG" {
    //             let mut body = json_message(err.message());
    //             if let Some(pg_err) = err.try_downcast_ref::<PgDatabaseError>() {
    //                 if let Some(hint) = pg_err.hint().map(|s| s.to_string()) {
    //                     body.0.hint = Some(hint);
    //                 }
    //                 if let Some(detail) = pg_err.detail() {
    //                     if let Ok(detail) = serde_json::from_str::<serde_json::Value>(detail) {
    //                         body.0.detail = Some(detail);
    //                     }
    //                 }
    //             }
    //             return (StatusCode::BAD_REQUEST, body);
    //         }
    //     };
    // };

    // convert all known PostgreSQL to validation errors

    if let sqlx::Error::Database(err) = err {
        if let Some(sqlstate_code) = err.code() {
            if sqlstate_code == UNIQUE_VIOLATION {
                if let Some(pg_err) = err.try_downcast_ref::<PgDatabaseError>() {
                    if let Some(detail) = pg_err.detail() {
                        if let Some((column_name, column_value)) =
                            extract_column_name_and_value(detail)
                        {
                            let unique_issue =
                                Issue::new("unique").with_param("value", column_value);
                            let error_details =
                                ErrorDetails::new().with_issue(column_name, unique_issue);
                            let error_body =
                                ErrorBody::new(INVALID_REASON).with_error_details(error_details);
                            return (StatusCode::BAD_REQUEST, Json(error_body));
                        }
                    }
                }
            }
        };
    };

    tracing::error!("sqlx error: {:?}", err);
    let error_body = json_message(INTERNAL_REASON);
    (StatusCode::INTERNAL_SERVER_ERROR, error_body)
}

fn extract_column_name_and_value(pg_detail_text: &str) -> Option<(String, String)> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        return Regex::new(r"Key \((?<key>.+)\)=\((?<value>.+)\) already exists").unwrap();
    });
    let Some(caps) = RE.captures(pg_detail_text) else {
        return None;
    };
    let key = caps.name("key").unwrap().as_str().to_owned();
    let value = caps.name("value").unwrap().as_str().to_owned();
    return Some((key, value));
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::prelude::*;

    use super::*;

    #[tokio::test]
    async fn extract_field_name_and_value_from_detail_works() -> Result<()> {
        let text = "Key (first_name_123)=(Mary-Jane m4ria) already exists.";
        let result = extract_column_name_and_value(text).unwrap();
        assert_eq!(result.0, "first_name_123");
        assert_eq!(result.1, "Mary-Jane m4ria");
        Ok(())
    }

    #[sqlx::test]
    async fn validation_database_errors_work(pool: sqlx::PgPool) -> Result<()> {
        sqlx::query!(
            // language=SQL
            r#"insert into "user" (username, password_hash) values ($1, 'lol') returning user_id;"#,
            "bob",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let sqlx_err = sqlx::query!(
            // language=SQL
            r#"insert into "user" (username, password_hash) values ($1, 'lol') returning user_id;"#,
            "bob",
        )
        .fetch_one(&pool)
        .await
        .unwrap_err();

        Error::Sqlx(sqlx_err)
            .assert_status(StatusCode::BAD_REQUEST)
            .assert_json(json!({
                "message": "Validation failed",
                "details": {
                    "username": [{
                        "code": "unique",
                        "params": {
                            "value": "bob",
                        },
                    }],
                },
            }));

        Ok(())
    }
}