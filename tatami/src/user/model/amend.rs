use uuid::Uuid;
use axum::http::StatusCode;
use crate::error;
use crate::user::model;
use crate::user::model::User;

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
        let maybe_user = model::describe(db, user_id).await?;
        return match maybe_user {
            Some(user) => Ok(user),
            None => Err((StatusCode::NOT_FOUND, "User not found".into())),
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
        .await
        .map_err(error::internal)?;
    Ok(User {
        user_id: record.user_id,
        username: record.username,
    })
}
