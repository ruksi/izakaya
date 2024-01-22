use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::prelude::*;
use crate::user;
use crate::user::User;
use crate::valid::Valid;

#[derive(Deserialize, Validate, Default, Debug, PartialEq, Eq)]
pub struct UserAmendment {
    #[validate(custom = "crate::valid::username")]
    pub username: Option<String>,
}

#[cfg(test)]
impl UserAmendment {
    pub fn new_valid<T>(username: Option<T>) -> Result<Valid<Self>>
    where
        T: Into<String>,
    {
        let declaration = Self {
            username: username.map(Into::into),
        };
        Ok(Valid::new(declaration)?)
    }
}

pub async fn amend(
    db: &sqlx::PgPool,
    user_id: Uuid,
    amendment: Valid<UserAmendment>,
) -> Result<User> {
    let amendment = amendment.into_inner();
    if amendment == UserAmendment::default() {
        let maybe_user = user::describe(db, user_id).await?;
        return match maybe_user {
            Some(user) => Ok(user),
            None => Err(Error::NotFound),
        };
    }
    let record = sqlx::query!(
        // language=SQL
        r#"update "user" u
                set
                    username = coalesce($1, u.username)
                where user_id = $2
                returning user_id, username;"#,
        amendment.username,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(User {
        user_id: record.user_id,
        username: record.username,
    })
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use serde_json::json;

    use crate::user::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn amend_works(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let bob = user::create(&pool, declaration).await?;

        let declaration = UserDeclaration::new_valid("alice", "alice@example.com", "p4ssw0rd")?;
        let alice = user::create(&pool, declaration).await?;

        let amendment = UserAmendment::new_valid(Some("bobby"))?;
        let bobby = amend(&pool, bob.user_id, amendment).await?;
        assert_eq!(bobby.user_id, bob.user_id);
        assert_eq!(bobby.username, "bobby");

        let re_bobby = user::describe(&pool, bob.user_id).await?.unwrap();
        assert_eq!(bobby, re_bobby);

        let re_alice = user::describe(&pool, alice.user_id).await?.unwrap();
        assert_eq!(re_alice.username, "alice");

        // nothing to change 🤷
        let amendment = UserAmendment::default();
        let am_bobby = amend(&pool, bob.user_id, Valid::new(amendment)?).await?;
        assert_eq!(bobby, am_bobby);

        // invalid change
        UserAmendment::new_valid(Some("bad alice"))
            .unwrap_err()
            .assert_json(json!({
                "message": "Validation failed",
                "details": {
                    "username": [{
                        "code": "regex",
                        "message": "Username must be aLpHaNuMeR1c, but may contain hyphens (-)",
                        "params": {"value": "bad alice"},
                    }],
                }
            }));

        // bad change to an existing username
        let amendment = UserAmendment::new_valid(Some("bobby"))?;
        amend(&pool, alice.user_id, amendment)
            .await
            .unwrap_err()
            .assert_status(StatusCode::BAD_REQUEST)
            .assert_json(json!({
                "message": "Validation failed",
                "details": {
                    "username": [{
                        "code": "unique",
                        "params": {"value": "bobby"},
                    }],
                }
            }));

        Ok(())
    }
}
