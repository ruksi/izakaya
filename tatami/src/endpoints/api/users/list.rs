use crate::endpoints::api::users::UserOut;
use axum::extract::State;
use axum::Json;

use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::UserFilter;

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<UserOut>>> {
    let users = user::list(&state.db_pool, UserFilter::default()).await?;
    let outbound = users.into_iter().map(Into::into).collect();
    Ok(Json(outbound))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{login, mock_server};
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn works_for_admin_users(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_admin_user(&db, &server).await?;

        let users = server.get("/api/users").await.json::<Vec<UserOut>>();
        assert_eq!(users.len(), 1); // the database admin we created in `as_website_admin`

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        user::create(&db, declaration).await?;

        let users = server.get("/api/users").await.json::<Vec<UserOut>>();
        assert_eq!(users.len(), 2); // admin and bob

        Ok(())
    }

    #[sqlx::test]
    async fn forbidden_for_normal_users(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        login::as_normal_user(&db, &server).await?;
        server.get("/api/users").await.assert_status_forbidden();
        Ok(())
    }
}
