use uuid::Uuid;

use crate::prelude::*;
use crate::user::User;

pub async fn describe(db: &sqlx::PgPool, user_id: Uuid) -> Result<Option<User>> {
    let result = sqlx::query!(
        // language=SQL
        r#"select user_id, username
               from "user"
               where user_id = $1;"#,
        user_id,
    )
    .fetch_optional(db)
    .await?;
    match result {
        Some(record) => Ok(Some(User {
            user_id: record.user_id,
            username: record.username,
        })),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use crate::user;
    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn describe_works(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let bob = user::create(&pool, declaration).await?;
        let re_bob = describe(&pool, bob.user_id).await?.unwrap();
        assert_eq!(bob, re_bob);
        Ok(())
    }

    #[sqlx::test]
    async fn describe_can_succeed_but_find_nothing(pool: sqlx::PgPool) -> Result<()> {
        assert!(describe(&pool, Uuid::new_v4()).await?.is_none());
        Ok(())
    }
}
