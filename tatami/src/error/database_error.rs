use axum::http::StatusCode;
use axum::Json;
use sqlx::postgres::PgDatabaseError;

use crate::error::utils::{reason, ErrorResponseBody, INTERNAL_REASON};

pub fn sqlx_error_to_response_tuple(err: &sqlx::Error) -> (StatusCode, Json<ErrorResponseBody>) {
    // check if the error code is our custom SQLSTATE error code
    // if it is, this is a safe, known error to expose to the client
    if let sqlx::Error::Database(err) = err {
        if let Some(sqlstate_code) = err.code() {
            if sqlstate_code == "WRONG" {
                let mut body = reason(err.message());
                if let Some(pg_err) = err.try_downcast_ref::<PgDatabaseError>() {
                    if let Some(hint) = pg_err.hint().map(|s| s.to_string()) {
                        body.0.hint = Some(hint);
                    }
                    if let Some(detail) = pg_err.detail() {
                        if let Ok(detail) = serde_json::from_str::<serde_json::Value>(detail) {
                            body.0.detail = Some(detail);
                        }
                    }
                }
                return (StatusCode::BAD_REQUEST, body);
            }
        };
    };

    tracing::error!("sqlx error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, reason(INTERNAL_REASON))
}
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    #[sqlx::test]
    async fn validation_database_errors_work(pool: sqlx::PgPool) -> Result<()> {
        let sqlx_err = sqlx::query!(
            // language=SQL
            r#"select * from validate_user_username($1);"#,
            "bad username",
        )
        .fetch_one(&pool)
        .await
        .unwrap_err();

        let body = sqlx_error_to_response_tuple(&sqlx_err).1;
        assert_eq!(body.0.reason, "Username is invalid");
        assert_eq!(
            body.0.hint,
            Some("Choose a username with only letters and numbers; you may use dashes (-) to separate words".into())
        );
        assert_eq!(
            body.0.detail,
            serde_json::json!({"name": "Username", "value": "bad username"}).into()
        );

        let err = Error::Database(sqlx_err);
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        assert_eq!(err.reason(), "Username is invalid");

        Ok(())
    }
}
