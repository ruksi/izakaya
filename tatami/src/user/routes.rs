use axum::{Json, Router};
use axum::routing::get;

use crate::state::AppState;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(index).post(create))
        .with_state(state)
}

async fn index() -> Json<Vec<()>> {
    Json(vec![])
}

#[derive(serde::Deserialize)]
struct CreateUserData {
    username: String,
    email: String,
    password: String,
}

async fn create(
    Json(data): Json<CreateUserData>,
) -> String {
    data.username
}

// #[cfg(test)]
// mod tests {
//     use axum_test::TestServer;
//     use serde_json::json;
//
//     use crate::state::AppState;
//
//     use super::*;
//
//     async fn test_cache_pool() -> deadpool_redis::Pool {
//         deadpool_redis::Config::from_url("redis://localhost:6379/9")
//             .create_pool(Some(deadpool_redis::Runtime::Tokio1))
//             .expect("Failed to create cache pool for tests")
//     }
//
//     #[sqlx::test]
//     async fn index_works(pool: sqlx::PgPool) {
//         let state = AppState { db_pool: pool, cache_pool: test_cache_pool().await };
//         let routes = router(state.clone());
//         let server = TestServer::new(routes).unwrap();
//         server
//             .get("/")
//             .await
//             .assert_json(&json!([]));
//     }
// }
