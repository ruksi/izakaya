use axum::http::StatusCode;
use sea_query::*;
use sea_query_binder::*;
use uuid::Uuid;

use crate::error;

// the user database table identifiers
#[derive(sea_query::Iden)]
enum User {
    Table,
    UserId,
    Username,
    // CreatedAt,
    // UpdatedAt,
    // PrimaryEmailId,
    // PasswordHash,
}

// one representation of the user
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct UserModel {
    pub user_id: Uuid,
    pub username: String,
}

impl UserModel {
    // the columns from the database that make up this representation of a user
    pub fn columns() -> impl IntoIterator<Item=impl IntoColumnRef> {
        [User::UserId, User::Username]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UserDeclaration {
    pub username: String,
    pub email: String,
}

impl UserDeclaration {
    pub fn new<T>(username: T, email: T) -> Self
        where T: Into<String>,
    {
        Self { username: username.into(), email: email.into() }
    }
}

pub async fn create(
    db: &sqlx::PgPool,
    declaration: UserDeclaration,
) -> Result<UserModel, (StatusCode, String)> {
    let mut tx = db.begin().await.map_err(error::internal)?;

    let (sql, values) = Query::insert()
        .into_table(User::Table)
        .columns([User::Username])
        .values_panic(vec![declaration.username.into()])
        .returning(Query::returning().columns(UserModel::columns()))
        .build_sqlx(PostgresQueryBuilder);
    let user: UserModel = sqlx::query_as_with(&sql, values)
        .fetch_one(&mut *tx)
        .await
        .map_err(error::internal)?;

    let email_id = sqlx::query_scalar!(
            // language=SQL
            r#"insert into user_email (user_id, address)
               values ($1, $2)
               returning email_id;"#,
            user.user_id,
            declaration.email,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(error::internal)?;

    sqlx::query!(
            // language=SQL
            r#"update "user"
               set primary_email_id = $2
               where user_id = $1"#,
            user.user_id,
            email_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(error::internal)?;

    tx.commit().await.map_err(error::internal)?;
    Ok(user)
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UserFilter {
    pub username: Option<String>,
}

pub async fn list(
    db: &sqlx::PgPool,
    filter: UserFilter,
) -> Result<Vec<UserModel>, (StatusCode, String)> {
    let mut query = Query::select();
    query
        .columns(UserModel::columns())
        .from(User::Table);

    if let Some(username) = filter.username {
        query.and_where(Expr::col(User::Username).eq(username));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let users = sqlx::query_as_with(&sql, values)
        .fetch_all(db)
        .await
        .map_err(error::internal)?;

    Ok(users)
}

pub async fn describe(
    db: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Option<UserModel>, (StatusCode, String)> {
    let (sql, values) = Query::select()
        .columns(UserModel::columns())
        .from(User::Table)
        .and_where(Expr::col(User::UserId).eq(user_id))
        .build_sqlx(PostgresQueryBuilder);
    let result = sqlx::query_as_with(&sql, values)
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
) -> Result<UserModel, (StatusCode, String)> {
    if amendment == UserAmendment::default() {
        // TODO: fix unwrap
        return Ok(describe(db, user_id).await?.unwrap());
    }

    let mut query = Query::update();
    query
        .table(User::Table)
        .and_where(Expr::col(User::UserId).eq(user_id))
        .returning(Query::returning().columns(UserModel::columns()));

    if let Some(username) = amendment.username {
        query.value(User::Username, username);
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
        .from_table(User::Table)
        .and_where(Expr::col(User::UserId).eq(user_id))
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

        let bob = create(&pool, UserDeclaration::new("bob", "bob@example.com")).await?;
        let alice = create(&pool, UserDeclaration::new("alice", "alice@example.com")).await?;
        assert_eq!(bob.username, "bob");
        assert_eq!(alice.username, "alice");
        assert_ne!(bob.user_id, alice.user_id);

        // trim
        let john = create(&pool, UserDeclaration::new("john ", "john@example.com")).await?;
        assert_eq!(john.username, "john");

        // existing username
        assert!(create(&pool, UserDeclaration::new("bob", "robert@example.com")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("bob ", "robert@example.com")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("Bob", "robert@example.com")).await.is_err());

        // existing email
        assert!(create(&pool, UserDeclaration::new("robert", "bob@example.com")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("bob", "bob+2@example.com")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("robert", "bob+2@example.com")).await.is_ok());

        // invalid username
        assert!(create(&pool, UserDeclaration::new("John Doe", "doe@exampe.com")).await.is_err());
        assert!(create(&pool, UserDeclaration::new("JohnDoe", "doe@example.com")).await.is_ok());

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
