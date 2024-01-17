use uuid::Uuid;

use crate::prelude::*;

pub async fn destroy(db: &sqlx::PgPool, user_id: Uuid) -> Result<()> {
    let count = sqlx::query_scalar!(
        // language=SQL
        r#"
        with deleted as (delete from "user" where user_id = $1 returning user_id) 
        select count(user_id) from deleted;
        "#,
        user_id,
    )
    .fetch_one(db)
    .await?;

    match count {
        Some(0) => Err(Error::NotFound),
        Some(1) => Ok(()),
        _ => Ok(()), // 🤷 I mean, it should match primary id
    }
}

#[cfg(test)]
mod tests {
    use crate::user::model::{create, describe, UserDeclaration};
    use axum::http::StatusCode;

    use super::*;

    #[sqlx::test]
    async fn destroy_works(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new("bob", "bob@example.com", "pw");
        let bob = create(&pool, declaration).await?;
        let declaration = UserDeclaration::new("alice", "alice@example.com", "pw");
        let alice = create(&pool, declaration).await?;

        destroy(&pool, bob.user_id).await?;
        assert!(describe(&pool, bob.user_id).await?.is_none());
        assert!(describe(&pool, alice.user_id).await?.is_some());
        Ok(())
    }

    #[sqlx::test]
    async fn destroy_fails_it_not_found(pool: sqlx::PgPool) -> Result<()> {
        let err = destroy(&pool, Uuid::new_v4()).await.unwrap_err();
        assert_eq!(err.response_tuple().0, StatusCode::NOT_FOUND);
        Ok(())
    }
}