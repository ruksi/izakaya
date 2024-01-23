use axum::extract::State;
use redis::AsyncCommands;

use crate::prelude::*;
use crate::state::AppState;

pub async fn healthz(State(state): State<AppState>) -> Result<String> {
    let from_db: String = sqlx::query_scalar("SELECT 'DATABASE OK'")
        .fetch_one(&state.db_pool)
        .await?;

    let mut redis = state.cache_pool.get().await?;
    redis.set("deadpool:test_key", "CACHE OK").await?;
    let from_cache: String = redis.get("deadpool:test_key").await?;

    Ok(format!("{}\n{}", from_db, from_cache))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::mock_server;

    use super::*;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        server
            .get("/healthz")
            .await
            .assert_text("DATABASE OK\nCACHE OK");
        Ok(())
    }
}
