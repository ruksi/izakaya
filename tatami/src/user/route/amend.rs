use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::state::AppState;
use crate::user::model;

#[derive(serde::Deserialize, Debug)]
pub struct AmendUserBody {
    username: Option<String>,
}

pub async fn amend(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<AmendUserBody>,
) -> Result<Json<model::User>, (StatusCode, String)> {
    let amendment = model::UserAmendment {
        username: Some(body.username.unwrap()),
    };
    let user = model::amend(&state.db_pool, user_id, amendment).await?;
    Ok(Json(user))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;

    use crate::test_utils::mock_state;
    use crate::user::model::UserDeclaration;
    use crate::user::route::router;

    use super::*;

    #[sqlx::test]
    async fn amend_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let user = model::create(
            &state.db_pool,
            UserDeclaration::new("bob", "bob@example.com", "pw"),
        )
        .await
        .unwrap();

        let user = server
            .patch(format!("/{}", user.user_id).as_str())
            .content_type(&"application/json")
            .json(&json!({
                "username": "bobby",
            }))
            .await
            .json::<model::User>();
        assert_eq!(user.username, "bobby");

        let user = model::describe(&state.db_pool, user.user_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user.username, "bobby");
    }
}
