use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::User;

pub async fn describe(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>> {
    let user = user::describe(&state.db_pool, user_id).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(Error::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use serde_json::json;

    use crate::test_utils::{as_website_admin, mock_server};
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        as_website_admin(&db, &server).await?;

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
            .json::<User>();
        assert_eq!(user, fetched_user);

        Ok(())
    }
}
