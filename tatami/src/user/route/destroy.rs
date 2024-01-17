use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::prelude::*;
use crate::state::AppState;
use crate::user::model;

pub async fn destroy(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>> {
    model::destroy(&state.db_pool, user_id).await?;
    Ok(Json(json!({"status": "ok"})))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use crate::test_utils::mock_state;
    use crate::user::model;
    use crate::user::model::UserDeclaration;
    use crate::user::route::router;

    use super::*;

    #[sqlx::test]
    async fn destroy_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let user = model::create(
            &state.db_pool,
            UserDeclaration::new("bob", "bob@example.com", "pw"),
        )
        .await
        .unwrap();
        server
            .delete(format!("/{}", user.user_id).as_str())
            .await
            .assert_json(&json!({"status": "ok"}));

        let response = server.get(format!("/{}", user.user_id).as_str()).await;
        response.assert_status(StatusCode::NOT_FOUND);
        // response.assert_text("User not found");
    }
}
