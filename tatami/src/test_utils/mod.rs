use crate::endpoints::router;
use crate::state::AppState;
use axum_test::{TestServer, TestServerConfig};

pub mod login;

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
