use axum::http::StatusCode;
use crate::{crypto, error};
use crate::user::model::User;

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
