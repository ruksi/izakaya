use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::{session_set_key, CurrentUser};
use crate::prelude::*;
use crate::scripts::RedisScripts;
use crate::state::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct ListSessionOut {
    pub access_token_prefix: String,
    pub used_at: Option<String>,
}

type RedisRecord = HashMap<String, String>;
impl From<RedisRecord> for ListSessionOut {
    fn from(record: RedisRecord) -> Self {
        let access_token_text = record
            .get("access_token")
            .map(|t| t.to_string())
            .unwrap_or_default();
        let access_token_prefix = access_token_text.chars().take(8).collect::<String>();
        let used_at = record.get("used_at").map(|t| t.to_string());
        ListSessionOut {
            access_token_prefix,
            used_at,
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Vec<ListSessionOut>>> {
    let user_id = current_user.user_id;

    let mut redis = state.cache_pool.get().await?;
    let sessions = redis.smembers_hgetall(session_set_key(user_id)).await?;

    let outbound = sessions.into_iter().map(Into::into).collect();
    Ok(Json(outbound))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::mock_server;
    use crate::user::{self, UserDeclaration};
    use serde_json::json;

    #[sqlx::test]
    async fn fails_without_auth(db: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&db).await;
        server
            .get("/api/sessions")
            .await
            .assert_status_unauthorized();
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
        let sessions = response.json::<Vec<ListSessionOut>>();
        assert_eq!(sessions.len(), 3);
        assert!(sessions
            .iter()
            .all(|session| session.access_token_prefix.len() == 8));

        // TODO: cleanup

        Ok(())
    }
}
