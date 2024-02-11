use axum_test::TestServer;
use serde_json::json;

use crate::user::UserDeclaration;
use crate::{error, user};

pub async fn as_admin_user(db: &sqlx::PgPool, server: &TestServer) -> error::Result<()> {
    let declaration = UserDeclaration::new_valid("admin-andy", "andy@example.com", "andyIsBest")?;
    let user = user::create(db, declaration).await?;

    // we don't have any way to promote superusers yet, so we'll just do it directly
    sqlx::query!(
        // language=SQL
        r#"update "user" set is_superuser = true where user_id = $1"#,
        user.user_id,
    )
    .execute(db)
    .await?;

    // this sets the auth cookie
    server
        .post("/log-in")
        .json(&json!({"username_or_email": "admin-andy", "password": "andyIsBest"}))
        .await
        .assert_status_ok();
    Ok(())
}

pub async fn as_normal_user(db: &sqlx::PgPool, server: &TestServer) -> error::Result<()> {
    let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "bobIsBest")?;
    user::create(db, declaration).await?;

    // this sets the auth cookie
    server
        .post("/log-in")
        .json(&json!({"username_or_email": "bob", "password": "bobIsBest"}))
        .await
        .assert_status_ok();
    Ok(())
}
