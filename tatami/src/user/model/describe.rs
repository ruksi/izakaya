use axum::http::StatusCode;
use uuid::Uuid;

use crate::error;
use crate::user::model::User;

pub async fn describe(
    db: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Option<User>, (StatusCode, String)> {
    let result = sqlx::query!(
            // language=SQL
            r#"select user_id, username
               from "user"
               where user_id = $1;"#,
            user_id,
        )
        .fetch_optional(db)
        .await
        .map_err(error::internal)?;
    match result {
        Some(record) => Ok(Some(User {
            user_id: record.user_id,
            username: record.username,
        })),
        None => Ok(None),
    }
}
