use axum::http::StatusCode;

use crate::error;
use crate::user::model::User;

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
