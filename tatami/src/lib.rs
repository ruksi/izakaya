use std::time::Duration;

use axum::extract::{MatchedPath, Request};
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use tokio::time::sleep;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub use crate::config::Config;
use crate::state::AppState;

mod auth;
mod config;
mod endpoints;
mod error;
mod prelude;
mod state;
mod user;
mod valid;

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

    let cookie_secret = auth::cookie::cookie_secret_from_seed(config.secret_key.clone());

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

    let app = app.route(
        "/favicon.ico",
        get(|| async { Redirect::permanent("/assets/favicon.ico") }),
    );
    let app = app.merge(endpoints::router(state.clone()));

    let app = app.layer(
        TraceLayer::new_for_http()
            .make_span_with(|req: &Request| {
                let method = req.method();
                let uri = req.uri();

                // this extension is set by axum
                let matched_path = req
                    .extensions()
                    .get::<MatchedPath>()
                    .map(|matched_path| matched_path.as_str());

                tracing::debug_span!("request", %method, %uri, matched_path)
            })
            .on_failure(()), // we trace::error the errors ourselves
    );

    app.with_state(state)
}
