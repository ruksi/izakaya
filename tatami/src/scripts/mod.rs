use std::collections::HashMap;

use once_cell::sync::Lazy;

// set a hash field to value if the hash exists
static HSET_X: Lazy<redis::Script> = Lazy::new(|| {
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

// get key references from a set and return the hashes
static SMEMBERS_HGETALL: Lazy<redis::Script> = Lazy::new(|| {
    return redis::Script::new(
        // language=Lua
        r#"
            local set = KEYS[1];
            local keys = redis.call('smembers', set);
            local result = {};
            for _, key in ipairs(keys) do
                local hash = redis.call('hgetall', key);
                if #hash > 0 then 
                    table.insert(result, hash);
                else
                    redis.call('srem', set, key);
                end
            end
            return result;
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
        HSET_X
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
        HSET_X
            .key(key)
            .arg(field1)
            .arg(value1)
            .arg(field2)
            .arg(value2)
            .invoke_async(self)
            .await
    }

    async fn smembers_hgetall<K>(
        &mut self,
        key: K,
    ) -> redis::RedisResult<Vec<HashMap<String, String>>>
    where
        Self: redis::aio::ConnectionLike + Sized,
        K: redis::ToRedisArgs,
    {
        SMEMBERS_HGETALL.key(key).invoke_async(self).await
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

    #[sqlx::test]
    async fn burr(db: sqlx::PgPool) -> Result<()> {
        let state = mock_state(&db).await;
        let mut redis = state.cache_pool.get().await?;

        let key1 = "tatami:tests:scripts:foo";
        let key2 = "tatami:tests:scripts:bar";
        let key3 = "tatami:tests:scripts:baz";
        let set_key = "tatami:tests:scripts:all";
        redis::pipe()
            .hset(key1, "x", "FOO")
            .hset(key2, "y", "BAR")
            .hset(key3, "z", "BAZ")
            .sadd(set_key, key1)
            .sadd(set_key, key2)
            .sadd(set_key, key3)
            .query_async(&mut redis)
            .await?;

        let hashes = redis.smembers_hgetall(set_key).await?;
        assert_eq!(hashes.len(), 3);
        assert!(hashes
            .iter()
            .any(|h| h.get("x") == Some(&"FOO".to_string())));
        assert!(hashes
            .iter()
            .any(|h| h.get("y") == Some(&"BAR".to_string())));
        assert!(hashes
            .iter()
            .any(|h| h.get("z") == Some(&"BAZ".to_string())));

        redis.del(key2).await?;
        let hashes = redis.smembers_hgetall(set_key).await?;
        assert_eq!(hashes.len(), 2);
        assert!(hashes
            .iter()
            .any(|h| h.get("x") == Some(&"FOO".to_string())));
        assert!(hashes
            .iter()
            .any(|h| h.get("z") == Some(&"BAZ".to_string())));

        // it also cleaned up the set
        let keys: Vec<String> = redis.smembers(set_key).await?;
        assert_eq!(keys.len(), 2);

        redis.del(key1).await?;
        redis.del(key3).await?;

        let hashes = redis.smembers_hgetall(set_key).await?;
        assert_eq!(hashes.len(), 0);

        Ok(())
    }
}
