use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::state::AppState;
use crate::user::model;

pub async fn describe(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<model::User>, (StatusCode, String)> {
    let user = model::describe(&state.db_pool, user_id).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => return Err((StatusCode::NOT_FOUND, "User not found".into())),
    }
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;

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
        response.assert_text("User not found");

        let user = model::create(&state.db_pool, UserDeclaration::new("bob", "bob@example.com", "pw")).await.unwrap();
        let re_user = server.get(format!("/{}", user.user_id).as_str()).await.json::<model::User>();
        assert_eq!(user, re_user);
    }
}