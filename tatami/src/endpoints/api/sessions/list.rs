use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::auth::{access_token_list_key, Visitor};
use crate::state::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub access_token_prefix: String,
}

pub async fn list(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
) -> crate::error::Result<Json<Value>> {
    let user_id = visitor.get_user_id_or_respond_unauthorized()?;

    let mut cache_conn = state.cache_pool.get().await?;
    let tokens: Vec<String> = deadpool_redis::redis::cmd("LRANGE")
        .arg(&[access_token_list_key(user_id), "0".into(), "-1".into()])
        .query_async(&mut cache_conn)
        .await?;

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
    async fn listing_own_sessions_requires_auth(pool: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&pool).await;
        let response = server.get("/api/sessions").await;
        response.assert_status(StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[sqlx::test]
    async fn listing_own_sessions_works(pool: sqlx::PgPool) -> Result<()> {
        let server = mock_server(&pool).await;
        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        user::create(&pool, declaration).await?;
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
