use axum::http::StatusCode;
use uuid::Uuid;

use crate::{crypto, error};

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UserDeclaration {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserDeclaration {
    pub fn new<T>(username: T, email: T, password: T) -> Self
        where T: Into<String>,
    {
        Self { username: username.into(), email: email.into(), password: password.into() }
    }
}

pub async fn create(
    db: &sqlx::PgPool,
    declaration: UserDeclaration,
) -> Result<User, (StatusCode, String)> {
    let password_hash = crypto::hash_password(declaration.password).await?;

    let mut tx = db.begin().await.map_err(error::internal)?;

    let user_record = sqlx::query!(
            // language=SQL
            r#"insert into "user" (username, password_hash)
               values ($1, $2)
               returning user_id, username;"#,
            declaration.username,
            password_hash,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(error::internal)?;

    let email_id = sqlx::query_scalar!(
            // language=SQL
            r#"insert into user_email (user_id, address)
               values ($1, $2)
               returning email_id;"#,
            user_record.user_id,
            declaration.email,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(error::internal)?;

    sqlx::query!(
            // language=SQL
            r#"update "user"
               set primary_email_id = $1
               where user_id = $2"#,
            email_id,
            user_record.user_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(error::internal)?;

    tx.commit().await.map_err(error::internal)?;

    let user_model = User {
        user_id: user_record.user_id,
        username: user_record.username,
    };
    Ok(user_model)
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UserFilter {
    pub username: Option<String>,
}

pub async fn list(
    db: &sqlx::PgPool,
    filter: UserFilter,
) -> Result<Vec<User>, (StatusCode, String)> {
    let mut query = sqlx::QueryBuilder::new(
        // language=SQL
        r#"select user_id, username from "user""#,
    );
    if filter != UserFilter::default() {
        query.push(" where");
        let mut conditions = query.separated(" and");
        if let Some(username) = filter.username {
            conditions.push(" username = ").push_bind_unseparated(username);
        }
    }

    let users: Vec<User> = query.build_query_as()
        .fetch_all(db)
        .await
        .map_err(error::internal)?;
    Ok(users)
}

pub async fn describe(
    db: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Option<User>, (StatusCode, String)> {
    let result = sqlx::query!(
            // language=SQL
            r#"select user_id, username
               from "user"
               where user_id = $1;"#,
            user_id,
        )
        .fetch_optional(db)
        .await
        .map_err(error::internal)?;
    match result {
        Some(record) => Ok(Some(User {
            user_id: record.user_id,
            username: record.username,
        })),
        None => Ok(None),
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UserAmendment {
    pub username: Option<String>,
}

pub async fn amend(
    db: &sqlx::PgPool,
    user_id: Uuid,
    amendment: UserAmendment,
) -> Result<User, (StatusCode, String)> {
    if amendment == UserAmendment::default() {
        // TODO: fix unwrap
        let maybe_user = describe(db, user_id).await?;
        match maybe_user {
            Some(user) => return Ok(user),
            None => return Err((StatusCode::NOT_FOUND, "User not found".into())),
        }
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
        .await
        .map_err(error::internal)?;
    Ok(User {
        user_id: record.user_id,
        username: record.username,
    })
}

pub async fn destroy(
    db: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<(), (StatusCode, String)> {
    sqlx::query!(
            // language=SQL
            r#"delete from "user"
               where user_id = $1;"#,
            user_id,
        )
        .execute(db)
        .await
        .map_err(error::internal)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn everything_works(pool: sqlx::PgPool) -> Result<(), (StatusCode, String)> {
        // list works on empty
        let users = list(&pool, UserFilter::default()).await?;
        assert_eq!(users.len(), 0);

        let bob = create(&pool, UserDeclaration::new("bob", "bob@example.com", "pw")).await?;
        let alice = create(&pool, UserDeclaration::new("alice", "alice@example.com", "pw")).await?;
        assert_eq!(bob.username, "bob");
        assert_eq!(alice.username, "alice");
        assert_ne!(bob.user_id, alice.user_id);

        // trim
        let john = create(&pool, UserDeclaration::new("john ", "john@example.com", "pw")).await?;
        assert_eq!(john.username, "john");

        // existing username
        assert!(create(&pool, UserDeclaration::new("bob", "robert@example.com", "pw")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("bob ", "robert@example.com", "pw")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("Bob", "robert@example.com", "pw")).await.is_err());

        // existing email
        assert!(create(&pool, UserDeclaration::new("robert", "bob@example.com", "pw")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("bob", "bob+2@example.com", "pw")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("robert", "bob+2@example.com", "pw")).await.is_ok());

        // invalid username
        assert!(create(&pool, UserDeclaration::new("John Doe", "doe@exampe.com", "pw")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("JohnDoe", "doe@example.com", "pw")).await.is_ok());

        // describe
        assert!(describe(&pool, Uuid::new_v4()).await?.is_none());
        let re_bob = describe(&pool, bob.user_id).await?.unwrap();
        assert_eq!(bob, re_bob);

        // amend
        let bobby = amend(&pool, bob.user_id, UserAmendment { username: Some("bobby".into()) }).await?;
        assert_eq!(bobby.user_id, bob.user_id);
        assert_eq!(bobby.username, "bobby");
        let re_bobby = describe(&pool, bob.user_id).await?.unwrap();
        assert_eq!(bobby, re_bobby);
        let re_alice = describe(&pool, alice.user_id).await?.unwrap();
        assert_eq!(re_alice.username, "alice");

        // amend but nothing to change
        assert!(amend(&pool, bob.user_id, UserAmendment::default()).await.is_ok());

        // destroy
        destroy(&pool, bob.user_id).await?;
        assert!(describe(&pool, bob.user_id).await?.is_none());
        assert!(describe(&pool, alice.user_id).await?.is_some());

        // destroy non-existing user
        assert!(destroy(&pool, Uuid::new_v4()).await.is_ok());

        // list
        let users = list(&pool, UserFilter::default()).await?;
        assert_eq!(users.len(), 4);

        // list with a filter
        let users = list(&pool, UserFilter { username: Some("joHndOe".into()) }).await?;
        assert_eq!(users.len(), 1);
        let users = list(&pool, UserFilter { username: Some("alice".into()) }).await?;
        assert_eq!(users.len(), 1);
        let users = list(&pool, UserFilter { username: Some("elvis".into()) }).await?;
        assert_eq!(users.len(), 0);

        Ok(())
    }
}
