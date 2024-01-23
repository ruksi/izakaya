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
    async fn works_if_nothing_is_found(db: sqlx::PgPool) -> Result<()> {
        assert!(describe(&db, Uuid::new_v4()).await?.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let bob = user::create(&db, declaration).await?;
        let re_bob = describe(&db, bob.user_id).await?.unwrap();
        assert_eq!(bob, re_bob);
        Ok(())
    }
}
