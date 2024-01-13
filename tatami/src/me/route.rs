use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use rand::Rng;
use serde_json::{json, Value};

use crate::{crypto, error};
use crate::auth::{bearer_key, Visitor};
use crate::state::AppState;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        // .route("/sign-up", post(signUp))
        .route("/login", post(login))
        .route("/logout", post(logout))
        // .route("/me", post(me))
        .with_state(state)
}

#[derive(serde::Deserialize, Debug)]
struct LoginBody {
    username_or_email: String,
    password: String,
}

async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginBody>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let result = sqlx::query!(
            // language=SQL
            r#"select user_id, password_hash
               from "user"
               left join user_email using (user_id)
               where username = $1
               or address = $1;"#,
            body.username_or_email,
        )
        .fetch_optional(&state.db_pool)
        .await
        .map_err(error::internal)?;

    let Some(record) = result else {
        return Err((StatusCode::UNAUTHORIZED, "Incorrect username or password.".into()));
    };

    let Some(password_hash) = record.password_hash else {
        return Err((StatusCode::UNAUTHORIZED, "Incorrect username or password.".into()));
    };

    let verification = crypto::verify_password(password_hash, body.password).await;
    if verification.is_err() {
        // probably "invalid password"
        return Err((StatusCode::UNAUTHORIZED, "Incorrect username or password.".into()));
    }

    // UUIDs have 16 bytes of randomness, and that is considered enough
    // for session identifiers if the random generator is secure enough.
    // Let's generate 64 bytes of randomness, just to be sure as I'm
    // not 100% sure what is used to generate the numbers on the server. 🤷
    let token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let mut cache_conn = state.cache_pool
        .get()
        .await
        .map_err(error::internal)?;
    deadpool_redis::redis::cmd("HSET")
        .arg(&[bearer_key(token.clone()), "user_id".into(), record.user_id.into()])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .map_err(error::internal)?;

    Ok(Json(json!({"bearer": token})))
}

async fn logout(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
) -> Result<Json<()>, (StatusCode, String)> {
    let Some(bearer) = visitor.bearer else {
        return Ok(Json(()));
    };

    let mut cache_conn = state.cache_pool
        .get()
        .await
        .map_err(error::internal)?;
    deadpool_redis::redis::cmd("DEL")
        .arg(&[bearer_key(bearer)])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .map_err(error::internal)?;

    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    use axum::http::header::AUTHORIZATION;
    use axum::http::HeaderValue;
    use axum_test::TestServer;
    use serde_json::json;

    use crate::auth::record_visit;
    use crate::state::AppState;
    use crate::user;
    use crate::user::model::UserDeclaration;

    use super::*;

    async fn test_cache_pool() -> deadpool_redis::Pool {
        deadpool_redis::Config::from_url("redis://localhost:6379/9")
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .expect("Failed to create cache pool for tests")
    }

    #[sqlx::test]
    async fn login_works(pool: sqlx::PgPool) -> Result<(), (StatusCode, String)> {
        let state = AppState { db_pool: pool, cache_pool: test_cache_pool().await };
        let db = &state.db_pool;
        user::model::create(db, UserDeclaration::new("bob", "bob@example.com", "bobIsBest")).await?;

        let server = TestServer::new(
            router(state.clone())
                .layer(axum::middleware::from_fn_with_state(state.clone(), record_visit))
        ).unwrap();

        // wrong password
        server
            .post("/login")
            .json(&json!({"username_or_email": "bob", "password": "bobIsBes"}))
            .await
            .assert_status_unauthorized();

        // wrong username
        server
            .post("/login")
            .json(&json!({"username_or_email": "bobby", "password": "bobIsBest"}))
            .await
            .assert_status_unauthorized();

        // works with username
        let response1 = server
            .post("/login")
            .json(&json!({"username_or_email": "bob", "password": "bobIsBest"}))
            .await;
        response1.assert_status_ok();
        let json1 = response1.json::<Value>();
        let token1 = json1.get("bearer").unwrap().as_str().unwrap();
        assert_eq!(token1.len(), 64);

        // works with email
        let response2 = server
            .post("/login")
            .json(&json!({"username_or_email": "bob@example.com", "password": "bobIsBest"}))
            .await;
        response2.assert_status_ok();
        let json2 = response2.json::<Value>();
        let token2 = json2.get("bearer").unwrap().as_str().unwrap();
        assert_eq!(token2.len(), 64);

        // double login is fine, you get two separate tokens
        assert_ne!(token1, token2);

        // logout with token is fine as ever
        server
            .post("/logout")
            .await
            .assert_status_ok();

        // logout with an unknown token is fine too
        server
            .post("/logout")
            .add_header(
                AUTHORIZATION,
                HeaderValue::from_static("Bearer unknown"),
            )
            .await
            .assert_status_ok();

        // logout with a valid token is the same
        // ... maybe should test that it got deleted
        server
            .post("/logout")
            .add_header(
                AUTHORIZATION,
                HeaderValue::from_str(format!("Bearer {}", token1).as_str()).unwrap(),
            )
            .await
            .assert_status_ok();

        // again is fine and responds the same
        server
            .post("/logout")
            .add_header(
                AUTHORIZATION,
                HeaderValue::from_str(format!("Bearer {}", token1).as_str()).unwrap(),
            )
            .await
            .assert_status_ok();

        // and again, just as fine
        server
            .post("/logout")
            .add_header(
                AUTHORIZATION,
                HeaderValue::from_str(format!("Bearer {}", token2).as_str()).unwrap(),
            )
            .await
            .assert_status_ok();

        Ok(())
    }
}
