use once_cell::sync::Lazy;

static HSETX: Lazy<redis::Script> = Lazy::new(|| {
    return redis::Script::new(
        // language=Lua
        r#"
                local hash = KEYS[1];
                if redis.call('exists', hash) ~= 0 then
                    return redis.call('hset', hash, unpack(ARGV));
                end
                return 0;
            "#,
    );
});

pub trait RedisScripts {
    async fn hset_x<K, F, V>(&mut self, key: K, field: F, value: V) -> redis::RedisResult<u8>
    where
        Self: redis::aio::ConnectionLike + Sized,
        K: redis::ToRedisArgs,
        F: redis::ToRedisArgs,
        V: redis::ToRedisArgs,
    {
        HSETX
            .key(key)
            .arg(field)
            .arg(value)
            .invoke_async(self)
            .await
    }

    // TODO: this is _not_ the proper way to do it but works until
    //       I figure out how to extend the Pipeline trait or something 🔧
    async fn hset_xx<K, F1, V1, F2, V2>(
        &mut self,
        key: K,
        field1: F1,
        value1: V1,
        field2: F2,
        value2: V2,
    ) -> redis::RedisResult<u8>
    where
        Self: redis::aio::ConnectionLike + Sized,
        K: redis::ToRedisArgs,
        F1: redis::ToRedisArgs,
        V1: redis::ToRedisArgs,
        F2: redis::ToRedisArgs,
        V2: redis::ToRedisArgs,
    {
        HSETX
            .key(key)
            .arg(field1)
            .arg(value1)
            .arg(field2)
            .arg(value2)
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
    async fn hsetx_works_with_single_field(db: sqlx::PgPool) -> Result<()> {
        let state = mock_state(&db).await;
        let mut redis = state.cache_pool.get().await?;

        let key = "tatami:tests:scripts:hsetx-single";

        // doesn't write if the hash doesn't exist
        let change_count = redis.hset_x(&key, "name", "Bob").await?;
        assert_eq!(change_count, 0);
        let name: Option<String> = redis.hget(&key, "name").await?;
        assert!(name.is_none());

        // writes if the hash exists
        redis.hset(&key, "unrelated", "thing").await?;
        let change_count = redis.hset_x(key, "name", "Mark").await?;
        assert_eq!(change_count, 1);
        let name: String = redis.hget(&key, "name").await?;
        assert_eq!(name, "Mark");

        // cleanup
        redis.del(&key).await?;

        Ok(())
    }

    #[sqlx::test]
    async fn hsetx_works_with_two_fields(db: sqlx::PgPool) -> Result<()> {
        let state = mock_state(&db).await;
        let mut redis = state.cache_pool.get().await?;

        let key = "tatami:tests:scripts:hsetx-duo";

        // doesn't write if the hash doesn't exist
        let change_count = redis.hset_xx(&key, "name", "Bob", "age", 22).await?;
        assert_eq!(change_count, 0);
        let name: Option<String> = redis.hget(&key, "name").await?;
        assert!(name.is_none());
        let age: Option<i8> = redis.hget(&key, "age").await?;
        assert!(age.is_none());

        // // writes if the hash exists
        redis.hset(&key, "unrelated", "thing").await?;
        let change_count = redis.hset_xx(key, "name", "Mark", "age", 33).await?;
        assert_eq!(change_count, 2);
        let name: String = redis.hget(&key, "name").await?;
        assert_eq!(name, "Mark");
        let name: i8 = redis.hget(&key, "age").await?;
        assert_eq!(name, 33);

        // cleanup
        redis.del(&key).await?;

        Ok(())
    }
}
