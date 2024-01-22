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
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use crate::handle::api::users::router;
    use crate::test_utils::mock_state;
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn destroy_works(pool: sqlx::PgPool) -> Result<()> {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let user = user::create(&state.db_pool, declaration).await.unwrap();
        server
            .delete(format!("/{}", user.user_id).as_str())
            .await
            .assert_json(&json!({"status": "ok"}));

        let response = server.get(format!("/{}", user.user_id).as_str()).await;
        response.assert_status(StatusCode::NOT_FOUND);
        // response.assert_text("User not found");

        Ok(())
    }
}
