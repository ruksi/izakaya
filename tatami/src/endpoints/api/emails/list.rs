use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::auth::CurrentUser;
use crate::prelude::*;
use crate::state::AppState;

#[derive(FromRow, Debug)]
pub struct Email {
    pub email_id: uuid::Uuid,
    pub email: String,
    pub is_primary: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListEmailOut {
    pub email_id: uuid::Uuid,
    pub email: String,
    pub is_primary: bool,
}

impl From<Email> for ListEmailOut {
    fn from(email: Email) -> Self {
        ListEmailOut {
            email_id: email.email_id,
            email: email.email,
            is_primary: email.is_primary.unwrap_or_default(),
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Vec<ListEmailOut>>> {
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

    let outbound = emails.into_iter().map(Into::into).collect();
    Ok(Json(outbound))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{login, mock_server};
    use serde_json::json;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_admin_user(&db, &server).await?; // so there exists unrelated emails
        login::as_normal_user(&db, &server).await?;

        server
            .post("/api/emails")
            .json(&json!({"email": "bob2@example.com"}))
            .await;

        let emails = server.get("/api/emails").await.json::<Vec<ListEmailOut>>();
        assert_eq!(emails.len(), 2);
        assert_eq!(emails.iter().filter(|e| e.is_primary).count(), 1);

        Ok(())
    }
}
