use crate::crypto;
use crate::prelude::*;
use crate::user::model::User;

#[derive(Debug, PartialEq, Eq)]
pub struct UserDeclaration {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserDeclaration {
    pub fn new<T>(username: T, email: T, password: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            username: username.into(),
            email: email.into(),
            password: password.into(),
        }
    }
}

pub async fn create(db: &sqlx::PgPool, declaration: UserDeclaration) -> Result<User> {
    let password_hash = crypto::hash_password(declaration.password).await?;

    let mut tx = db.begin().await?;

    let user_record = sqlx::query!(
        // language=SQL
        r#"insert into "user" (username, password_hash)
               values ($1, $2)
               returning user_id, username;"#,
        declaration.username,
        password_hash,
    )
    .fetch_one(&mut *tx)
    .await?;

    let email_id = sqlx::query_scalar!(
        // language=SQL
        r#"insert into user_email (user_id, address)
               values ($1, $2)
               returning email_id;"#,
        user_record.user_id,
        declaration.email,
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        // language=SQL
        r#"update "user"
               set primary_email_id = $1
               where user_id = $2"#,
        email_id,
        user_record.user_id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let user_model = User {
        user_id: user_record.user_id,
        username: user_record.username,
    };
    Ok(user_model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn create_works(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new("bob", "bob@example.com", "pw");
        let bob = create(&pool, declaration).await?;

        let declaration = UserDeclaration::new("4lice", "alice@example.com", "pw");
        let alice = create(&pool, declaration).await?;

        assert_eq!(bob.username, "bob");
        assert_eq!(alice.username, "4lice");
        assert_ne!(bob.user_id, alice.user_id);
        Ok(())
    }

    #[sqlx::test]
    async fn create_trims_username(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new("john ", "john@example.com", "pw");
        let john = create(&pool, declaration).await?;
        assert_eq!(john.username, "john");
        Ok(())
    }

    #[sqlx::test]
    async fn create_rejects_existing_username(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new("bob", "bob@example.com", "pw");
        create(&pool, declaration).await?;

        for username in ["bob", "bob ", "Bob"] {
            let declaration = UserDeclaration::new(username, "robert@example.com", "pw");
            let err = create(&pool, declaration).await.unwrap_err();
            assert_eq!(err.reason(), "Username is already in use");
        }
        Ok(())
    }

    #[sqlx::test]
    async fn create_rejects_existing_email(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new("bob", "bob@example.com", "pw");
        create(&pool, declaration).await?;
        for email in ["bob@example.com", " bob@example.com"] {
            let declaration = UserDeclaration::new("robert", email, "pw");
            let err = create(&pool, declaration).await.unwrap_err();
            assert_eq!(err.reason(), "Email is already in use");
        }
        Ok(())
    }

    #[sqlx::test]
    async fn create_rejects_invalid_username(pool: sqlx::PgPool) -> Result<()> {
        // TODO: reject , "-Doe", "Doe-"
        for username in ["John Doe", "John_Doe", "JohnDoe!"] {
            let declaration = UserDeclaration::new(username, "doe@example.com", "pw");
            let err = create(&pool, declaration).await.unwrap_err();
            assert_eq!(err.reason(), "Username is invalid");
        }
        Ok(())
    }
}
