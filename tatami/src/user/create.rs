use serde::Deserialize;
use validator::Validate;

use crate::auth::hash_password;
use crate::prelude::*;
use crate::user::User;
use crate::valid::Valid;

#[derive(Deserialize, Validate, Debug, Clone, PartialEq, Eq)]
pub struct UserDeclaration {
    #[validate(custom = "crate::valid::username")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom = "crate::valid::password")]
    pub password: String,
}

#[cfg(test)]
impl UserDeclaration {
    pub fn new_valid<T: Into<String>>(username: T, email: T, password: T) -> Result<Valid<Self>> {
        let declaration = Self {
            username: username.into(),
            email: email.into(),
            password: password.into(),
        };
        Valid::new(declaration)
    }
}

pub async fn create(db: &sqlx::PgPool, declaration: Valid<UserDeclaration>) -> Result<User> {
    let declaration = declaration.into_inner();
    let password_hash = hash_password(declaration.password).await?;

    let mut tx = db.begin().await?;

    let user = sqlx::query_as!(
        User,
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
        r#"insert into user_email (user_id, email)
            values ($1, $2)
            returning email_id;"#,
        user.user_id,
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
        user.user_id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let bob = create(&db, declaration).await?;

        let declaration = UserDeclaration::new_valid("4lice", "alice@example.com", "p4ssw0rd")?;
        let alice = create(&db, declaration).await?;

        let declaration = UserDeclaration::new_valid("John-Doe", "john@example.com", "p4ssw0rd")?;
        let john = create(&db, declaration).await?;

        assert_eq!(bob.username, "bob");
        assert_eq!(alice.username, "4lice");
        assert_eq!(john.username, "John-Doe");

        assert_ne!(bob.user_id, alice.user_id);
        assert_ne!(alice.user_id, john.user_id);
        assert_ne!(john.user_id, bob.user_id);
        Ok(())
    }

    #[sqlx::test]
    async fn fails_on_existing_username(db: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        create(&db, declaration).await?;

        for username in ["bob", "Bob"] {
            let declaration =
                UserDeclaration::new_valid(username, "robert@example.com", "p4ssw0rd")?;
            let err = create(&db, declaration).await.unwrap_err();
            err.assert_json(json!({
                "message": "Validation failed",
                "issues": {
                    "username": [{
                        "code": "unique",
                        "details": {"value": username},
                    }],
                }
            }));
        }
        Ok(())
    }

    #[sqlx::test]
    async fn fails_on_existing_email(db: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        create(&db, declaration).await?;
        for email in ["bob@example.com", "BOB@example.com"] {
            let declaration = UserDeclaration::new_valid("robert", email, "p4ssw0rd")?;
            let err = create(&db, declaration).await.unwrap_err();
            err.assert_json(json!({
                "message": "Validation failed",
                "issues": {
                    "email": [{
                        "code": "unique",
                        "details": {"value": email},
                    }],
                }
            }));
        }
        Ok(())
    }

    #[tokio::test]
    async fn fails_on_invalid_email() -> Result<()> {
        for email in ["", " ", "bob", "@bob", "bob@gmail.com ", " bob@gmail.com"] {
            let err = UserDeclaration::new_valid("bob", email, "p4ssw0rd").unwrap_err();
            err.assert_json(json!({
                "message": "Validation failed",
                "issues": {
                    "email": [{
                        "code": "email",
                        "details": {"value": email},
                    }],
                }
            }));
        }
        Ok(())
    }

    #[tokio::test]
    async fn fails_on_invalid_username() -> Result<()> {
        for username in ["John Doe", "John_Doe", "JohnDoe!", "-doe", "doe-"] {
            let err =
                UserDeclaration::new_valid(username, "doe@example.com", "p4ssw0rd").unwrap_err();
            err.assert_json(json!({
                "message": "Validation failed",
                "issues": {
                    "username": [{
                        "code": "regex",
                        "message": "Username must be aLpHaNuMeR1c, but may contain hyphens (-)",
                        "details": {"value": username},
                    }],
                }
            }));
        }
        Ok(())
    }

    #[tokio::test]
    async fn fails_on_short_usernames() -> Result<()> {
        for username in ["", "a", "jk"] {
            let err =
                UserDeclaration::new_valid(username, "a@example.com", "p4ssw0rd").unwrap_err();
            err.assert_json(json!({
                "message": "Validation failed",
                "issues": {
                    "username": [{
                        "code": "length",
                        "details": {
                            "max": 32,
                            "min": 3,
                            "value": username,
                        },
                    }],
                }
            }));
        }
        Ok(())
    }
}
