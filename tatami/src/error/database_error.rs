use axum::http::StatusCode;
use axum::Json;

use crate::error::utils::{reason, ErrorBody, INTERNAL_REASON};

pub fn sqlx_error_to_response_tuple(err: &sqlx::Error) -> (StatusCode, Json<ErrorBody>) {
    // check if the error code is our custom SQLSTATE error code
    // if it is, this is a safe, known error to expose to the client
    if let sqlx::Error::Database(err) = err {
        if let Some(sqlstate_code) = err.code() {
            if sqlstate_code == "WRONG" {
                return (StatusCode::BAD_REQUEST, reason(err.message()));
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
            r#"select * from validate_user_username($1, null);"#,
            "bad username",
        )
        .fetch_one(&pool)
        .await
        .unwrap_err();

        let err = Error::Database(sqlx_err);
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        assert_eq!(err.reason(), "username_must_be_only_letters_and_dashes");

        Ok(())
    }
}
