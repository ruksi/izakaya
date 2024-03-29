use crate::endpoints::api::users::UserOut;
use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::UserAmendment;
use crate::valid::Valid;

pub async fn amend(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    amendment: Valid<UserAmendment>,
) -> Result<Json<UserOut>> {
    let user = user::amend(&state.db_pool, user_id, amendment).await?;
    Ok(Json(user.into()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_utils::{login, mock_server};
    use crate::user;
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_admin_user(&db, &server).await?;

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let user = user::create(&db, declaration).await?;

        let user = server
            .patch(format!("/api/users/{}", user.user_id).as_str())
            .content_type("application/json")
            .json(&json!({
                "username": "bobby",
            }))
            .await
            .json::<UserOut>();
        assert_eq!(user.username, "bobby");

        let user = user::describe(&db, user.user_id).await?;
        assert_eq!(user.unwrap().username, "bobby");

        Ok(())
    }
}
