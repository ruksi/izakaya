use axum::extract::State;
use axum::Json;

use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::{User, UserFilter};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<User>>> {
    let users = user::list(&state.db_pool, UserFilter::default()).await?;
    Ok(Json(users))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;

    use crate::handle::api::users::router;
    use crate::test_utils::mock_state;
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn list_handler_works(pool: sqlx::PgPool) -> Result<()> {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        server.get("/").await.assert_json(&json!([]));

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        user::create(&state.db_pool, declaration).await.unwrap();

        let users = server.get("/").await.json::<Vec<User>>();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "bob");

        Ok(())
    }
}
