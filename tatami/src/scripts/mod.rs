use once_cell::sync::Lazy;

pub trait RedisScripts {
    async fn hsetx<K, F, V>(&mut self, key: K, field: F, value: V) -> redis::RedisResult<u8>
    where
        Self: redis::aio::ConnectionLike + Sized,
        K: redis::ToRedisArgs,
        F: redis::ToRedisArgs,
        V: redis::ToRedisArgs,
    {
        static SCRIPT: Lazy<redis::Script> = Lazy::new(|| {
            return redis::Script::new(
                // language=Lua
                r#"
                    local hash = KEYS[1];
                    if redis.call('exists', hash) ~= 0 then 
                        return redis.call('hset', hash, ARGV[1], ARGV[2]);
                    end
                    return 0;
                "#,
            );
        });
        SCRIPT
            .key(key)
            .arg(field)
            .arg(value)
            .invoke_async(self)
            .await
    }
}

impl RedisScripts for redis::aio::Connection {}

#[cfg(test)]
mod tests {
    use redis::AsyncCommands;

    use crate::prelude::*;
    use crate::test_utils::mock_state;

    use super::*;

    #[sqlx::test]
    async fn hsetx_works(db: sqlx::PgPool) -> Result<()> {
        let state = mock_state(&db).await;
        let mut redis = state.cache_pool.get().await?;

        let key = "tatami:tests:scripts:hsetx";

        // doesn't write if the hash doesn't exist
        let change_count = redis.hsetx(&key, "name", "Bob").await?;
        assert_eq!(change_count, 0);
        let name: Option<String> = redis.hget(&key, "name").await?;
        assert!(name.is_none());

        // writes if the hash exists
        redis.hset(&key, "unrelated", "thing").await?;
        let change_count = redis.hsetx(key, "name", "Mark").await?;
        assert_eq!(change_count, 1);
        let name: String = redis.hget(&key, "name").await?;
        assert_eq!(name, "Mark");

        // cleanup
        redis.del(&key).await?;

        Ok(())
    }
}
