use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;

use crate::{crypto, error};
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
) -> Result<Json<()>, (axum::http::StatusCode, String)> {
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

    // TODO: create session and session token
    // TODO: store session and session token in Redis
    // TODO: return session token cookie

    Ok(Json(()))
}

async fn logout(
    State(_state): State<AppState>,
    // Json(body): Json<crate::user::route::CreateUserBody>,
) -> Result<Json<()>, (axum::http::StatusCode, String)> {
    // TODO: if no session token in cookie, return
    // TODO: delete session from Redis
    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;

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

        let server = TestServer::new(router(state.clone())).unwrap();

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
        server
            .post("/login")
            .json(&json!({"username_or_email": "bob", "password": "bobIsBest"}))
            .await
            .assert_status_ok();

        // works with email
        server
            .post("/login")
            .json(&json!({"username_or_email": "bob@example.com", "password": "bobIsBest"}))
            .await
            .assert_status_ok();

        // TODO: check that there is some session cookie
        Ok(())
    }
}
