use axum_test::{TestServer, TestServerConfig};

use crate::endpoints::router;
use crate::state::AppState;

// mock a server with all endpoint routes and auth middleware enabled
pub async fn mock_server(db_pool: &sqlx::PgPool) -> TestServer {
    let state = mock_state(db_pool.clone()).await;
    let config = TestServerConfig::builder().save_cookies().build(); // <- automatically use cookies
    TestServer::new_with_config(router(state.clone()), config).unwrap()
}

pub async fn mock_state(db_pool: sqlx::PgPool) -> AppState {
    let cache_pool = mock_cache_pool().await;
    let cookie_secret = axum_extra::extract::cookie::Key::generate();
    AppState {
        db_pool,
        cache_pool,
        cookie_secret,
    }
}

pub async fn mock_cache_pool() -> deadpool_redis::Pool {
    // TODO: get the test Redis URL from somewhere
    let pool = deadpool_redis::Config::from_url("redis://localhost:6379/9")
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create cache pool for tests");
    // should be FLUSHDB here?
    pool
}
