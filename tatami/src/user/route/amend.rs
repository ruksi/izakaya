use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::prelude::*;
use crate::state::AppState;
use crate::user::model;
use crate::user::model::UserAmendment;
use crate::valid::Valid;

pub async fn amend(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    amendment: Valid<UserAmendment>,
) -> Result<Json<model::User>> {
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
    async fn amend_route_works(pool: sqlx::PgPool) -> Result<()> {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let user = model::create(&state.db_pool, declaration).await?;

        let user = server
            .patch(format!("/{}", user.user_id).as_str())
            .content_type(&"application/json")
            .json(&json!({
                "username": "bobby",
            }))
            .await
            .json::<model::User>();
        assert_eq!(user.username, "bobby");

        let user = model::describe(&state.db_pool, user.user_id).await?;
        assert_eq!(user.unwrap().username, "bobby");

        Ok(())
    }
}
