use std::time::Duration;

use axum::extract::State;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use tokio::time::sleep;
use tower_http::services::ServeDir;

use crate::config::Config;
use crate::prelude::*;
use crate::session::cookie::cookie_secret_from_seed;
use crate::state::AppState;

mod api;
mod auth;
pub mod config;
mod crypto;
mod error;
mod prelude;
mod session;
mod state;
mod user;

#[cfg(test)]
mod test_utils;

pub async fn get_app<S>(config: &Config) -> Router<S> {
    // Railway private networks take time to initialize on deployment,
    // and application crashes make it re-initialize, so we have to wait
    // https://docs.railway.app/reference/private-networking#caveats
    if std::env::var("RAILWAY_ENVIRONMENT_NAME").is_ok() {
        tracing::debug!("Railway detected, waiting for private network to initialize...");
        sleep(Duration::from_secs(5)).await;
    }

    let db_pool = sqlx::pool::PoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Can't connect to database");

    // sqlx locks the migration table, so it's fine to run on multiple instances
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Can't run migrations");

    let cache_cfg = deadpool_redis::Config::from_url(&config.cache_url);
    let cache_pool = cache_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Can't create cache pool");

    let cookie_secret = cookie_secret_from_seed(config.secret_key.clone());

    let state = AppState {
        db_pool,
        cache_pool,
        cookie_secret,
    };
    root_router(state)
}

fn root_router<S>(state: AppState) -> Router<S> {
    let app = Router::new();

    // the ServeDir path is relative to where the binary is run,
    // thus this is assuming working directory is `ryokan/tatami`
    let app = app.nest_service("/assets", ServeDir::new("./assets"));

    let app = app.route("/", get(index));
    let app = app.route(
        "/favicon.ico",
        get(|| async { Redirect::permanent("/assets/favicon.ico") }),
    );
    let app = app.route("/healthz", get(healthz));

    let app = app.nest("/api", api::router(state.clone()));
    let app = app.nest("/sessions", session::route::router(state.clone()));

    let app = app.layer(axum::middleware::from_fn_with_state(
        state.clone(),
        crate::auth::record_visit,
    ));
    app.with_state(state)
}

async fn index() -> &'static str {
    "Hello, World!"
}

async fn healthz(State(state): State<AppState>) -> Result<String> {
    let from_db: String = sqlx::query_scalar("SELECT 'DATABASE OK'")
        .fetch_one(&state.db_pool)
        .await?;

    let mut cache_conn = state.cache_pool.get().await?;
    deadpool_redis::redis::cmd("SET")
        .arg(&["deadpool:test_key", "CACHE OK"])
        .query_async::<_, ()>(&mut cache_conn)
        .await?;
    let from_cache: String = deadpool_redis::redis::cmd("GET")
        .arg(&["deadpool:test_key"])
        .query_async(&mut cache_conn)
        .await?;

    Ok(format!("{}\n{}", from_db, from_cache))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;

    use crate::test_utils;

    use super::*;

    #[sqlx::test]
    async fn health_endpoint(pool: sqlx::PgPool) {
        let state = test_utils::mock_state(pool).await;
        let routes = root_router(state.clone());
        let server = TestServer::new(routes).unwrap();
        server
            .get("/healthz")
            .await
            .assert_text("DATABASE OK\nCACHE OK");
    }
}
