#[derive(Clone)]
pub struct AppState {
    pub config: crate::config::Config,
    pub db_pool: sqlx::pool::Pool<sqlx::Postgres>,
    pub cache_pool: deadpool_redis::Pool,
}

impl AppState {
    pub fn new(
        config: crate::config::Config,
        db_pool: sqlx::pool::Pool<sqlx::Postgres>,
        cache_pool: deadpool_redis::Pool,
    ) -> Self {
        Self {
            config,
            db_pool,
            cache_pool,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_utils::mock_state;

    #[sqlx::test]
    async fn mock_state_database_works(db: sqlx::PgPool) -> Result<()> {
        let state = mock_state(&db).await;
        let from_db: String = sqlx::query_scalar("SELECT 'DATABASE OK'")
            .fetch_one(&state.db_pool)
            .await?;
        assert_eq!(from_db, "DATABASE OK");
        Ok(())
    }

    #[sqlx::test]
    async fn mock_state_cache_works(db: sqlx::PgPool) -> Result<()> {
        let state = mock_state(&db).await;
        let mut redis = state.cache_pool.get().await?;
        let (from_cache,): (String,) = redis::pipe()
            .set("tatami:tests:test_key", "CACHE OK")
            .ignore()
            .get("tatami:tests:test_key")
            .del("tatami:tests:test_key")
            .ignore()
            .query_async(&mut redis)
            .await?;
        assert_eq!(from_cache, "CACHE OK");
        Ok(())
    }
}
