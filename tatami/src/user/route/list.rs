use axum::extract::State;
use axum::Json;

use crate::prelude::*;
use crate::state::AppState;
use crate::user::model;
use crate::user::model::UserFilter;

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<model::User>>> {
    let users = model::list(&state.db_pool, UserFilter::default()).await?;
    Ok(Json(users))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;

    use crate::test_utils::mock_state;
    use crate::user::model;
    use crate::user::model::UserDeclaration;
    use crate::user::route::router;

    #[sqlx::test]
    async fn list_handler_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        server.get("/").await.assert_json(&json!([]));

        let declaration = UserDeclaration::new("bob", "bob@example.com", "pw");
        model::create(&state.db_pool, declaration).await.unwrap();

        let users = server.get("/").await.json::<Vec<model::User>>();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "bob");
    }
}
