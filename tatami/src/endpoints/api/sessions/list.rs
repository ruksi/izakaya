use axum::extract::State;
use axum::{Extension, Json};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::auth::{session_list_key, CurrentUser};
use crate::state::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub access_token_prefix: String,
}

pub async fn list(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> crate::error::Result<Json<Value>> {
    let user_id = current_user.user_id;

    let mut redis = state.cache_pool.get().await?;
    let tokens: Vec<String> = redis.lrange(session_list_key(user_id), 0, -1).await?;

    // we don't want to expose the full tokens
    let sessions: Vec<Session> = tokens
        .into_iter()
        .map(|token| token[..8].to_string())
        .map(|prefix| Session {
            access_token_prefix: prefix,
        })
        .collect();

    Ok(Json(json!(sessions)))
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

        let response = server.get("/api/sessions").await;
        let sessions = response.json::<Vec<Session>>();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].access_token_prefix.len(), 8);

        Ok(())
    }
}
