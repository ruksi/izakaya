use axum::http::StatusCode;
use axum::Json;
use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::postgres::PgDatabaseError;

use crate::error::response::*;

// https://www.postgresql.org/docs/current/errcodes-appendix.html
pub const UNIQUE_VIOLATION: &str = "23505";

pub fn sqlx_error_to_response_tuple(err: &sqlx::Error) -> (StatusCode, Json<ErrorOut>) {
    // convert common PostgreSQL errors to validation errors
    if let sqlx::Error::Database(db_err) = err {
        if let Some(sqlstate_code) = db_err.code() {
            if sqlstate_code == UNIQUE_VIOLATION {
                if let Some(pg_err) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    if let Some(detail) = pg_err.detail() {
                        if let Some((col_name, col_value)) = parse_unique_violation(detail) {
                            let issue = IssueOut::new("unique").with_param("value", col_value);
                            let issue_map = IssueMapOut::new().with_field_issue(col_name, issue);
                            let outbound = ErrorOut::new(REASON_INVALID).with_issue_map(issue_map);
                            return (StatusCode::BAD_REQUEST, Json(outbound));
                        }
                    }
                }
            }
        };
    };

    tracing::error!("sqlx error: {:?}", err);
    let outbound = error_message(REASON_INTERNAL);
    (StatusCode::INTERNAL_SERVER_ERROR, outbound)
}

fn parse_unique_violation(pg_detail_text: &str) -> Option<(String, String)> {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Key \((?<key>.+)\)=\((?<value>.+)\) already exists").unwrap());
    let Some(caps) = RE.captures(pg_detail_text) else {
        return None;
    };
    let key = caps.name("key").unwrap().as_str().to_owned();
    let value = caps.name("value").unwrap().as_str().to_owned();
    Some((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn extract_field_name_and_value_from_detail_works() -> Result<()> {
        let text = "Key (first_name_123)=(Mary-Jane m4ria) already exists.";
        let result = parse_unique_violation(text).unwrap();
        assert_eq!(result.0, "first_name_123");
        assert_eq!(result.1, "Mary-Jane m4ria");
        Ok(())
    }

    #[sqlx::test]
    async fn validation_database_errors_work(db: sqlx::PgPool) -> Result<()> {
        sqlx::query!(
            // language=SQL
            r#"insert into "user" (username, password_hash) values ($1, 'lol') returning user_id;"#,
            "bob",
        )
        .fetch_one(&db)
        .await
        .unwrap();

        let sqlx_err = sqlx::query!(
            // language=SQL
            r#"insert into "user" (username, password_hash) values ($1, 'lol') returning user_id;"#,
            "bob",
        )
        .fetch_one(&db)
        .await
        .unwrap_err();

        Error::Sqlx(sqlx_err)
            .assert_status(StatusCode::BAD_REQUEST)
            .assert_json(json!({
                "message": "Validation failed",
                "issues": {
                    "username": [{
                        "code": "unique",
                        "details": {
                            "value": "bob",
                        },
                    }],
                },
            }));

        Ok(())
    }
}
