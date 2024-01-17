use axum::extract::State;
use axum::Json;

use crate::prelude::*;
use crate::state::AppState;
use crate::user::model;
use crate::user::model::UserDeclaration;

#[derive(serde::Deserialize, Debug)]
pub struct CreateUserBody {
    username: String,
    email: String,
    password: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateUserBody>,
) -> Result<Json<model::User>> {
    let declaration = UserDeclaration::new(body.username, body.email, body.password);
    let user = model::create(&state.db_pool, declaration).await?;
    Ok(Json(user))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;
    use uuid::Uuid;

    use crate::test_utils::mock_state;
    use crate::user::route::router;

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

        let user = response.json::<model::User>();
        assert_eq!(user.username, "bob");
        assert_ne!(user.user_id, Uuid::nil());
    }
}
