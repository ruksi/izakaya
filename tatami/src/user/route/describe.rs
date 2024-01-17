use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::prelude::*;
use crate::state::AppState;
use crate::user::model;

pub async fn describe(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<model::User>> {
    let user = model::describe(&state.db_pool, user_id).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(Error::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    use crate::test_utils::mock_state;
    use crate::user::model::UserDeclaration;
    use crate::user::route::router;

    use super::*;

    #[sqlx::test]
    async fn describe_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let response = server.get(format!("/{}", Uuid::new_v4()).as_str()).await;
        response.assert_status(StatusCode::NOT_FOUND);
        response.assert_json(&json!({"reason": "the resource was not found"}));

        let declaration = UserDeclaration::new("bob", "bob@example.com", "pw");
        let user = model::create(&state.db_pool, declaration).await.unwrap();
        let fetched_user = server
            .get(format!("/{}", user.user_id).as_str())
            .await
            .json::<model::User>();
        assert_eq!(user, fetched_user);
    }
}
