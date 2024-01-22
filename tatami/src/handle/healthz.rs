use axum::extract::State;

use crate::prelude::*;
use crate::state::AppState;

pub async fn healthz(State(state): State<AppState>) -> Result<String> {
    let from_db: String = sqlx::query_scalar("SELECT 'DATABASE OK'")
        .fetch_one(&state.db_pool)
        .await?;

    let mut cache_conn = state.cache_pool.get().await?;
    deadpool_redis::redis::cmd("SET")
        .arg(&["deadpool:test_key", "CACHE OK"])
        .query_async::<_, ()>(&mut cache_conn)
        .await?;
    let from_cache: String = deadpool_redis::redis::cmd("GET")
        .arg(&["deadpool:test_key"])
        .query_async(&mut cache_conn)
        .await?;

    Ok(format!("{}\n{}", from_db, from_cache))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;

    use crate::handle::router;
    use crate::test_utils::mock_state;

    use super::*;

    #[sqlx::test]
    async fn health_endpoint(pool: sqlx::PgPool) -> Result<()> {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();
        server
            .get("/healthz")
            .await
            .assert_text("DATABASE OK\nCACHE OK");
        Ok(())
    }
}
