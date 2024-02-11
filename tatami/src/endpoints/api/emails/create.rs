use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::auth::CurrentUser;
use crate::prelude::*;
use crate::state::AppState;
use crate::valid::Valid;

#[derive(Deserialize, Validate, Debug, Clone, PartialEq, Eq)]
pub struct CreateEmailBody {
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateEmailResponse {
    pub email_id: uuid::Uuid,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    body: Valid<CreateEmailBody>,
) -> Result<Json<CreateEmailResponse>> {
    let user_id = current_user.user_id;
    let body = body.into_inner();
    let email_id = sqlx::query_scalar!(
        // language=SQL
        r#"insert into user_email (user_id, email)
               values ($1, $2)
               returning email_id;"#,
        user_id,
        body.email,
    )
    .fetch_one(&state.db_pool)
    .await?;
    Ok(Json(CreateEmailResponse { email_id }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{login, mock_server};
    use serde_json::json;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_normal_user(&db, &server).await?;
        let new_email_id = server
            .post("/api/emails")
            .json(&json!({"email": "bob2@example.com"}))
            .await
            .json::<CreateEmailResponse>()
            .email_id;
        assert_ne!(new_email_id, uuid::Uuid::nil());
        Ok(())
    }

    #[sqlx::test]
    async fn fails_on_duplicate(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_normal_user(&db, &server).await?;
        let response = server
            .post("/api/emails")
            .json(&json!({"email": "bob@example.com"}))
            .await;
        response.assert_status_bad_request();
        let json = response.json::<serde_json::Value>();
        assert_eq!(
            json.get("details")
                .unwrap()
                .get("email")
                .unwrap()
                .get(0)
                .unwrap()
                .get("code")
                .unwrap(),
            "unique"
        );
        Ok(())
    }
}
