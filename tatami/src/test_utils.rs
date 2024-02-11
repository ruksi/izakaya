use axum_test::{TestServer, TestServerConfig};
use serde_json::json;

use crate::endpoints::router;
use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::UserDeclaration;

// create and login as a website admin to get the authentication cookie set
pub async fn as_website_admin(db: &sqlx::PgPool, server: &TestServer) -> Result<()> {
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

    server
        .post("/log-in")
        .json(&json!({"username_or_email": "admin-andy", "password": "andyIsBest"}))
        .await
        .assert_status_ok();
    Ok(())
}

// mock a server with all endpoint routes and auth middleware enabled
pub async fn mock_server(db: &sqlx::PgPool) -> TestServer {
    let state = mock_state(db).await;
    let config = TestServerConfig::builder().save_cookies().build(); // <- automatically use cookies
    TestServer::new_with_config(router(state.clone()), config).unwrap()
}

pub async fn mock_state(db: &sqlx::PgPool) -> AppState {
    let cache_pool = mock_cache_pool().await;
    let config = crate::config::Config::new_for_tests();
    AppState::new(config, db.clone(), cache_pool)
}

pub async fn mock_cache_pool() -> deadpool_redis::Pool {
    // TODO: get the test Redis URL from somewhere

    // should be FLUSHDB here?
    deadpool_redis::Config::from_url("redis://localhost:6379/9")
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create cache pool for tests")
}
