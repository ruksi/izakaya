use crate::state::AppState;

pub async fn mock_cache_pool() -> deadpool_redis::Pool {
    // TODO: get the test Redis URL from somewhere
    let pool = deadpool_redis::Config::from_url("redis://localhost:6379/9")
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create cache pool for tests");
    // should be FLUSHDB here?
    pool
}

pub async fn mock_state(db_pool: sqlx::PgPool) -> AppState {
    let cache_pool = mock_cache_pool().await;
    AppState {
        db_pool,
        cache_pool,
    }
}
