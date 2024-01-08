use axum::http::StatusCode;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use uuid::Uuid;

use crate::error;

#[sea_query::enum_def]
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UserDeclaration {
    pub username: String,
}

pub async fn create(
    db: &sqlx::PgPool,
    declaration: UserDeclaration,
) -> Result<User, (StatusCode, String)> {
    let (sql, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns([UserIden::Username])
        .values_panic(vec![declaration.username.into()])
        .returning(Query::returning().columns([UserIden::UserId, UserIden::Username]))
        .build_sqlx(PostgresQueryBuilder);
    let user = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(error::internal)?;
    Ok(user)
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UserFilter {
    pub username: Option<String>,
}

pub async fn list(
    db: &sqlx::PgPool,
    filter: UserFilter,
) -> Result<Vec<User>, (StatusCode, String)> {
    let mut query = Query::select();

    query
        .column(UserIden::UserId)
        .column(UserIden::Username)
        .from(UserIden::Table);

    if filter != UserFilter::default() {
        query.and_where(
            Expr::col(UserIden::Username).eq(filter.username.unwrap())
        );
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let users = sqlx::query_as_with(&sql, values.clone())
        .fetch_all(db)
        .await
        .map_err(error::internal)?;

    Ok(users)
}

pub async fn describe(
    db: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Option<User>, (StatusCode, String)> {
    let (sql, values) = Query::select()
        .columns([UserIden::UserId, UserIden::Username])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::UserId).eq(user_id))
        .build_sqlx(PostgresQueryBuilder);
    let result = sqlx::query_as_with(&sql, values.clone())
        .fetch_optional(db)
        .await
        .map_err(error::internal)?;
    match result {
        Some(user) => Ok(Some(user)),
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
        return Ok(describe(db, user_id).await?.unwrap());
    }

    let mut query = Query::update();
    query
        .table(UserIden::Table)
        .and_where(Expr::col(UserIden::UserId).eq(user_id))
        .returning(Query::returning().columns([UserIden::UserId, UserIden::Username]));

    if let Some(username) = amendment.username {
        query.value(UserIden::Username, username);
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let user = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(error::internal)?;
    Ok(user)
}

pub async fn destroy(
    db: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<(), (StatusCode, String)> {
    let (sql, values) = Query::delete()
        .from_table(UserIden::Table)
        .and_where(Expr::col(UserIden::UserId).eq(user_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values)
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

        let bob = create(&pool, UserDeclaration { username: "bob".into() }).await?;
        let alice = create(&pool, UserDeclaration { username: "alice".into() }).await?;
        assert_eq!(bob.username, "bob");
        assert_eq!(alice.username, "alice");
        assert_ne!(bob.user_id, alice.user_id);

        // trim
        let john = create(&pool, UserDeclaration { username: "john ".into() }).await?;
        assert_eq!(john.username, "john");

        // existing username
        assert!(create(&pool, UserDeclaration { username: "bob".into() }).await.is_err());
        assert!(create(&pool, UserDeclaration { username: "bob ".into() }).await.is_err());
        assert!(create(&pool, UserDeclaration { username: "Bob".into() }).await.is_err());

        // invalid username
        assert!(create(&pool, UserDeclaration { username: "John Doe".into() }).await.is_err());
        assert!(create(&pool, UserDeclaration { username: "JohnDoe".into() }).await.is_ok());

        // describe
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
        assert_eq!(users.len(), 3);

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
