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
    use axum::http::StatusCode;

    use crate::user;
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn destroy_works(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let bob = user::create(&pool, declaration).await?;
        let declaration = UserDeclaration::new_valid("alice", "alice@example.com", "p4ssw0rd")?;
        let alice = user::create(&pool, declaration).await?;

        destroy(&pool, bob.user_id).await?;
        assert!(user::describe(&pool, bob.user_id).await?.is_none());
        assert!(user::describe(&pool, alice.user_id).await?.is_some());
        Ok(())
    }

    #[sqlx::test]
    async fn destroy_fails_it_not_found(pool: sqlx::PgPool) -> Result<()> {
        let err = destroy(&pool, Uuid::new_v4()).await.unwrap_err();
        assert_eq!(err.response_tuple().0, StatusCode::NOT_FOUND);
        Ok(())
    }
}
