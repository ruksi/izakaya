use axum::http::StatusCode;
use uuid::Uuid;

use crate::error;

pub async fn destroy(
    db: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<(), (StatusCode, String)> {
    sqlx::query!(
            // language=SQL
            r#"delete from "user"
               where user_id = $1;"#,
            user_id,
        )
        .execute(db)
        .await
        .map_err(error::internal)?;
    Ok(())
}
