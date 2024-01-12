use axum::{Json, Router};
use axum::extract::State;
use axum::routing::post;

use crate::state::AppState;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        // .route("/sign-up", post(signUp))
        .route("/login", post(login))
        .route("/logout", post(logout))
        // .route("/me", post(me))
        .with_state(state)
}

async fn login(
    State(_state): State<AppState>,
    // Json(body): Json<crate::user::route::CreateUserBody>,
) -> Result<Json<()>, (axum::http::StatusCode, String)> {
    // TODO: if logged in, return
    // TODO: verify the credentials
    // TODO: create session
    // TODO: store session in Redis
    // TODO: return session cookie
    Ok(Json(()))
}

async fn logout(
    State(_state): State<AppState>,
    // Json(body): Json<crate::user::route::CreateUserBody>,
) -> Result<Json<()>, (axum::http::StatusCode, String)> {
    // TODO: if not logged in, return
    // TODO: delete session from Redis
    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    // use serde_json::json;
    // use uuid::Uuid;

    use crate::state::AppState;
    use super::*;

    async fn test_cache_pool() -> deadpool_redis::Pool {
        deadpool_redis::Config::from_url("redis://localhost:6379/9")
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .expect("Failed to create cache pool for tests")
    }

    #[sqlx::test]
    async fn login_works(pool: sqlx::PgPool) {
        let state = AppState { db_pool: pool, cache_pool: test_cache_pool().await };
        // user::model::create(
        //     &state.db_pool,
        //     crate::user::model::UserDeclaration { username: "alpha".into() },
        // ).await.unwrap();

        let _server = TestServer::new(router(state.clone())).unwrap();
        // let response = server
        //     .post("/")
        //     .content_type(&"application/json")
        //     .json(&json!({
        //         "username": "bob",
        //         "email": "bob@example.com",
        //         "password": "bobIsBest",
        //     }))
        //     .await;
        // let user = response.json::<user::model::UserModel>();
        // assert_eq!(user.username, "bob");
        // assert_ne!(user.user_id, Uuid::nil());
    }
}
