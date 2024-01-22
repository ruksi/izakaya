use axum::extract::State;
use axum::Json;

use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::{User, UserDeclaration};
use crate::valid::Valid;

pub async fn create(
    State(state): State<AppState>,
    declaration: Valid<UserDeclaration>,
) -> Result<Json<User>> {
    let user = user::create(&state.db_pool, declaration).await?;
    Ok(Json(user))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;
    use uuid::Uuid;

    use crate::handle::api::users::router;
    use crate::test_utils::mock_state;

    use super::*;

    #[sqlx::test]
    async fn create_handler_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let response = server
            .post("/")
            .json(&json!({
                "username": "bob",
                "email": "bob@example.com",
                "password": "bobIsBest",
            }))
            .await;

        let user = response.json::<User>();
        assert_eq!(user.username, "bob");
        assert_ne!(user.user_id, Uuid::nil());
    }

    #[sqlx::test]
    async fn create_handler_handles_validation_errors(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();
        server
            .post("/")
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
    }
}
