use axum::extract::State;

use crate::prelude::*;
use crate::state::AppState;

const HEALTH_CACHE_KEY: &str = "tatami:healthz";

pub async fn healthz(State(state): State<AppState>) -> Result<String> {
    let db_status = check_db(&state).await?;
    let cache_status = check_cache(&state).await?;
    Ok(format!("{}\n{}", db_status, cache_status))
}

pub async fn check_db(state: &AppState) -> Result<String> {
    let db_status: String = sqlx::query_scalar("SELECT 'DATABASE OK'")
        .fetch_one(&state.db_pool)
        .await?;
    Ok(db_status)
}

pub async fn check_cache(state: &AppState) -> Result<String> {
    #[rustfmt::skip]
    let (cache_status,): (String,) = redis::pipe()
        .set(HEALTH_CACHE_KEY, "CACHE OK").ignore()
        .get(HEALTH_CACHE_KEY)
        .del(HEALTH_CACHE_KEY).ignore()
        .query_async(&mut state.cache_pool.get().await?)
        .await?;
    Ok(cache_status)
}

#[cfg(test)]
mod tests {
    use crate::test_utils::mock_server;

    use super::*;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        let response = server.get("/healthz").await;
        response.assert_text("DATABASE OK\nCACHE OK");
        Ok(())
    }
}
