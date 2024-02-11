use axum::extract::{Path, State};
use axum::{Extension, Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::CurrentUser;
use crate::prelude::*;
use crate::state::AppState;

pub async fn destroy(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(email_id): Path<Uuid>,
) -> Result<Json<Value>> {
    let user_id = current_user.user_id;

    // NB: won't allow deleting if primary because of foreign key constraint
    sqlx::query!(
        // language=SQL
        r#"delete from user_email
            where user_id = $1 
            and email_id = $2;"#,
        user_id,
        email_id,
    )
    .execute(&state.db_pool)
    .await?;

    Ok(Json(json!({"status": "ok"})))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoints::api::emails::create::CreateEmailOut;
    use crate::endpoints::api::emails::list::ListEmailOut;
    use crate::test_utils::{login, mock_server};

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_normal_user(&db, &server).await?;

        let secondary_email_id = server
            .post("/api/emails")
            .json(&json!({"email": "bob2@example.com"}))
            .await
            .json::<CreateEmailOut>()
            .email_id;

        let emails = server.get("/api/emails").await.json::<Vec<ListEmailOut>>();
        assert_eq!(emails.len(), 2);

        server
            .delete(format!("/api/emails/{secondary_email_id}").as_str())
            .await
            .assert_status_ok();

        let emails = server.get("/api/emails").await.json::<Vec<ListEmailOut>>();
        assert_eq!(emails.len(), 1);

        Ok(())
    }

    #[sqlx::test]
    async fn cant_delete_primary_email(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_normal_user(&db, &server).await?;

        let emails = server.get("/api/emails").await.json::<Vec<ListEmailOut>>();
        assert_eq!(emails.len(), 1);
        let primary_email_id = emails[0].email_id;

        server
            .delete(format!("/api/emails/{primary_email_id}").as_str())
            .await
            .assert_status_failure(); // TODO: something better than 500 here

        let emails = server.get("/api/emails").await.json::<Vec<ListEmailOut>>();
        assert_eq!(emails.len(), 1);

        Ok(())
    }
}
