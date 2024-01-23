use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::auth::{access_token_list_key, Visitor};
use crate::state::AppState;

#[derive(Deserialize, Serialize, Debug)]
struct MySession {
    access_token_prefix: String,
}

pub async fn list_sessions(
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
    let sessions: Vec<MySession> = tokens
        .into_iter()
        .map(|token| token[..8].to_string())
        .map(|prefix| MySession {
            access_token_prefix: prefix,
        })
        .collect();

    Ok(Json(json!(sessions)))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::{TestServer, TestServerConfig};
    use serde_json::json;

    use crate::endpoints::api::my::list_sessions::MySession;
    use crate::endpoints::router;
    use crate::prelude::*;
    use crate::test_utils::mock_state;
    use crate::user::{self, UserDeclaration};

    #[sqlx::test]
    async fn listing_own_sessions_requires_auth(pool: sqlx::PgPool) -> Result<()> {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();
        let response = server.get("/api/my/sessions").await;
        response.assert_status(StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[sqlx::test]
    async fn listing_own_sessions_works(pool: sqlx::PgPool) -> Result<()> {
        let state = mock_state(pool).await;
        let config = TestServerConfig::builder().save_cookies().build(); // <- automatically use cookies
        let server = TestServer::new_with_config(router(state.clone()), config).unwrap();

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        user::create(&state.db_pool, declaration).await?;
        server
            .post("/log-in")
            .json(&json!({"username_or_email": "bob", "password": "p4ssw0rd"}))
            .await
            .assert_status_ok();

        let response = server.get("/api/my/sessions").await;
        let sessions = response.json::<Vec<MySession>>();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].access_token_prefix.len(), 8);

        Ok(())
    }
}
