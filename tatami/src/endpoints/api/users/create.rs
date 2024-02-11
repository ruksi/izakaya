use crate::endpoints::api::users::UserOut;
use axum::extract::State;
use axum::Json;

use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::UserDeclaration;
use crate::valid::Valid;

pub async fn create(
    State(state): State<AppState>,
    declaration: Valid<UserDeclaration>,
) -> Result<Json<UserOut>> {
    let user = user::create(&state.db_pool, declaration).await?;
    Ok(Json(user.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{login, mock_server};
    use serde_json::json;
    use uuid::Uuid;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_admin_user(&db, &server).await?;

        let response = server
            .post("/api/users")
            .json(&json!({
                "username": "bob",
                "email": "bob@example.com",
                "password": "bobIsBest",
            }))
            .await;

        response.assert_status_ok();
        let user = response.json::<UserOut>();
        assert_eq!(user.username, "bob");
        assert_ne!(user.user_id, Uuid::nil());
        Ok(())
    }

    #[sqlx::test]
    async fn fails_on_validation_errors(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_admin_user(&db, &server).await?;

        server
            .post("/api/users")
            .json(&json!({
                "username": "john doe",
                "email": "john@example.com",
                "password": "johnIsBest",
            }))
            .await
            .assert_json(&json!({
                "message": "Validation failed",
                "details": {
                    "username": [{
                        "code": "regex",
                        "message": "Username must be aLpHaNuMeR1c, but may contain hyphens (-)",
                        "params": {
                            "value": "john doe"
                        }
                    }]
                }
            }));
        Ok(())
    }
}
