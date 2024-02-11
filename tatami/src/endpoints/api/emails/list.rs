use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::auth::CurrentUser;
use crate::prelude::*;
use crate::state::AppState;

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct Email {
    pub email_id: uuid::Uuid,
    pub email: String,
    pub is_primary: Option<bool>,
}

pub async fn list(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Vec<Email>>> {
    let user_id = current_user.user_id;
    let emails = sqlx::query_as!(
        Email,
        // language=SQL
        r#"select email_id, email, u.primary_email_id = email_id as is_primary
           from user_email
           left join "user" u using (user_id)
           where user_id = $1
           order by user_email.created_at, email_id;"#,
        user_id,
    )
    .fetch_all(&state.db_pool)
    .await?;
    Ok(Json(emails))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{login, mock_server};
    use serde_json::{json, Value};

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_admin_user(&db, &server).await?; // so there exists unrelated emails
        login::as_normal_user(&db, &server).await?;

        server
            .post("/api/emails")
            .json(&json!({"email": "bob2@example.com"}))
            .await;

        let json = server.get("/api/emails").await.json::<Value>();
        assert_eq!(json.as_array().unwrap().len(), 2);

        Ok(())
    }
}
