use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::prelude::*;
use crate::state::AppState;
use crate::user;

pub async fn destroy(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>> {
    user::destroy(&state.db_pool, user_id).await?;
    Ok(Json(json!({"status": "ok"})))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{as_website_admin, mock_server};
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        as_website_admin(&db, &server).await?;

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let user = user::create(&db, declaration).await.unwrap();
        server
            .get(format!("/api/users/{}", user.user_id).as_str())
            .await
            .assert_status_ok();
        server
            .delete(format!("/api/users/{}", user.user_id).as_str())
            .await
            .assert_json(&json!({"status": "ok"}));
        server
            .get(format!("/api/users/{}", user.user_id).as_str())
            .await
            .assert_status_not_found();

        Ok(())
    }
}
