use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::endpoints::api::users::UserOut;
use crate::prelude::*;
use crate::state::AppState;
use crate::user;

pub async fn describe(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserOut>> {
    let user = user::describe(&state.db_pool, user_id).await?;
    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err(Error::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use serde_json::json;

    use crate::test_utils::{login, mock_server};
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_admin_user(&db, &server).await?;

        let response = server
            .get(format!("/api/users/{}", Uuid::new_v4()).as_str())
            .await;
        response.assert_status(StatusCode::NOT_FOUND);
        response.assert_json(&json!({"message": "The thing doesn't exist"}));

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let user = user::create(&db, declaration).await.unwrap();
        let fetched_user = server
            .get(format!("/api/users/{}", user.user_id).as_str())
            .await
            .json::<UserOut>();
        assert_eq!(user.user_id, fetched_user.user_id);
        assert_eq!(user.username, fetched_user.username);

        Ok(())
    }
}
