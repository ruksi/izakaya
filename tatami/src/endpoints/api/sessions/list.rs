use axum::extract::State;
use axum::{Extension, Json};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::auth::{session_key, session_list_key, CurrentUser};
use crate::state::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct PublicSession {
    pub access_token_prefix: String,
    pub used_at: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> crate::error::Result<Json<Value>> {
    let user_id = current_user.user_id;

    let mut redis = state.cache_pool.get().await?;
    let access_tokens: Vec<String> = redis.lrange(session_list_key(user_id), 0, -1).await?;

    let mut commands = redis::pipe().to_owned();
    for token in &access_tokens {
        commands = commands.hgetall(session_key(token)).to_owned();
    }
    let sessions: Vec<HashMap<String, String>> = commands.query_async(&mut redis).await?;

    // TODO: we _could_ remove the empty hashes from the Redis list here...

    // we don't want to expose the full tokens
    let public_sessions: Vec<PublicSession> = sessions
        .into_iter()
        .filter(|session| !session.is_empty())
        .map(|session| {
            let access_token_text = session
                .get("access_token")
                .map(|t| t.to_string())
                .unwrap_or_default();
            let access_token_prefix = access_token_text.chars().take(8).collect::<String>();

            let used_at = session.get("used_at").map(|t| t.to_string());

            PublicSession {
                access_token_prefix,
                used_at,
            }
        })
        .collect();

    Ok(Json(json!(public_sessions)))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use serde_json::json;

    use crate::prelude::*;
    use crate::test_utils::mock_server;
    use crate::user::{self, UserDeclaration};

    use super::*;

    #[sqlx::test]
    async fn fails_without_auth(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        let response = server.get("/api/sessions").await;
        response.assert_status(StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        user::create(&db, declaration).await?;

        server
            .post("/log-in")
            .json(&json!({"username_or_email": "bob", "password": "p4ssw0rd"}))
            .await
            .assert_status_ok();

        server
            .post("/log-in")
            .json(&json!({"username_or_email": "bob@example.com", "password": "p4ssw0rd"}))
            .await
            .assert_status_ok();

        server
            .post("/log-in")
            .json(&json!({"username_or_email": "bob", "password": "p4ssw0rd"}))
            .await
            .assert_status_ok();

        let response = server.get("/api/sessions").await;
        let sessions = response.json::<Vec<PublicSession>>();
        assert_eq!(sessions.len(), 3);
        assert!(sessions
            .iter()
            .all(|session| session.access_token_prefix.len() == 8));

        Ok(())
    }
}
