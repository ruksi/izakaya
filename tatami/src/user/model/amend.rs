use uuid::Uuid;

use crate::prelude::*;
use crate::user::model;
use crate::user::model::User;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UserAmendment {
    pub username: Option<String>,
}

pub async fn amend(db: &sqlx::PgPool, user_id: Uuid, amendment: UserAmendment) -> Result<User> {
    if amendment == UserAmendment::default() {
        // TODO: fix unwrap
        let maybe_user = model::describe(db, user_id).await?;
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
    use crate::user::model::{amend, create, describe, UserDeclaration};

    use super::*;

    #[sqlx::test]
    async fn amend_works(pool: sqlx::PgPool) -> Result<()> {
        let declaration = UserDeclaration::new("bob", "bob@example.com", "pw");
        let bob = create(&pool, declaration).await?;

        let declaration = UserDeclaration::new("alice", "alice@example.com", "pw");
        let alice = create(&pool, declaration).await?;

        let amendment = UserAmendment {
            username: Some("bobby".into()),
        };
        let bobby = amend(&pool, bob.user_id, amendment).await?;
        assert_eq!(bobby.user_id, bob.user_id);
        assert_eq!(bobby.username, "bobby");

        let re_bobby = describe(&pool, bob.user_id).await?.unwrap();
        assert_eq!(bobby, re_bobby);

        let re_alice = describe(&pool, alice.user_id).await?.unwrap();
        assert_eq!(re_alice.username, "alice");

        // nothing to change 🤷
        let amendment = UserAmendment::default();
        let am_bobby = amend(&pool, bob.user_id, amendment).await?;
        assert_eq!(bobby, am_bobby);

        // invalid change
        let amendment = UserAmendment {
            username: Some("bad alice".into()),
        };
        let err = amend(&pool, alice.user_id, amendment).await.unwrap_err();
        assert_eq!(err.reason(), "Username is invalid");

        // bad change to an existing username
        let amendment = UserAmendment {
            username: Some("bobby".into()),
        };
        let err = amend(&pool, alice.user_id, amendment).await.unwrap_err();
        assert_eq!(err.reason(), "Username is already in use");

        Ok(())
    }
}
