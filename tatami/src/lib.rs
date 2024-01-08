use std::time::Duration;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::Router;
use axum::routing::get;
use tokio::time::sleep;
use tower_http::services::ServeDir;

use crate::config::Config;
use crate::state::AppState;

pub mod config;
mod error;
mod user;
mod state;
mod api;

pub async fn get_app<S>(config: &Config) -> Router<S> {

    // Railway private networks take time to initialize on deployment,
    // and application crashes make it re-initialize, so we have to wait
    // https://docs.railway.app/reference/private-networking#caveats
    if let Ok(_) = std::env::var("RAILWAY_ENVIRONMENT_NAME") {
        tracing::debug!("Railway detected, waiting for private network to initialize...");
        sleep(Duration::from_secs(5)).await;
    }

    let db_pool = sqlx::pool::PoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Can't connect to database");

    // sqlx migrations lock the migrations table so it's fine to run on multiple instances
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Can't run migrations");

    let cache_cfg = deadpool_redis::Config::from_url(&config.cache_url);
    let cache_pool = cache_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Can't create cache pool");

    let state = AppState { db_pool, cache_pool };

    root_router(state)
}

fn root_router<S>(state: AppState) -> Router<S> {
    let app = Router::new();

    // the ServeDir path is relative to where the binary is run,
    // thus this is assuming working directory is `ryokan/tatami`
    let app = app.nest_service("/assets", ServeDir::new("./assets"));

    let app = app.route("/", get(index));
    let app = app.route("/favicon.ico", get(|| async { Redirect::permanent("/assets/favicon.ico") }));
    let app = app.route("/healthz", get(healthz));

    let app = app.nest("/api", api::router(state.clone()));
    let app = app.with_state(state);
    app
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn healthz(
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    let from_db: String = sqlx::query_scalar("SELECT 'DATABASE OK'")
        .fetch_one(&state.db_pool)
        .await
        .map_err(error::internal)?;

    let mut cache_conn = state.cache_pool
        .get()
        .await
        .map_err(error::internal)?;
    deadpool_redis::redis::cmd("SET")
        .arg(&["deadpool/test_key", "CACHE OK"])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .map_err(error::internal)?;
    let from_cache: String = deadpool_redis::redis::cmd("GET")
        .arg(&["deadpool/test_key"])
        .query_async(&mut cache_conn)
        .await
        .map_err(error::internal)?;

    Ok(format!("{}\n{}", from_db, from_cache))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;

    use super::*;

    async fn test_cache_pool() -> deadpool_redis::Pool {
        // TODO: get the test Redis URL from somewhere
        let pool = deadpool_redis::Config::from_url("redis://localhost:6379/9")
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .expect("Failed to create cache pool for tests");
        // TODO: is flushing the test Redis database necessary?
        // let mut conn = pool
        //     .get()
        //     .await
        //     .expect("Failed to get cache connection for tests");
        // deadpool_redis::redis::cmd("FLUSHDB")
        //     .query_async::<_, ()>(&mut conn)
        //     .await
        //     .expect("Failed to flush cache for tests");
        pool
    }

    #[sqlx::test]
    async fn health_endpoint(pool: sqlx::PgPool) {
        let state = AppState { db_pool: pool, cache_pool: test_cache_pool().await };
        let routes = root_router(state.clone());
        let server = TestServer::new(routes).unwrap();
        server
            .get("/healthz")
            .await
            .assert_text("DATABASE OK\nCACHE OK");
    }
}
