use std::time::Duration;

use axum::extract::{MatchedPath, Request};
use axum::http::{header, HeaderValue, Method};
use axum::response::Redirect;
use axum::Router;
use axum::routing::get;
use tokio::time::sleep;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub use crate::config::Config;
use crate::state::AppState;

mod auth;
mod config;
mod endpoints;
mod error;
mod prelude;
mod scripts;
mod state;
mod user;
mod valid;

#[cfg(test)]
mod test_utils;

pub async fn get_app<S: Clone + Send + Sync + 'static>(config: Config) -> Router<S> {
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

    let state = AppState::new(db_pool, cache_pool, cookie_secret);
    let mut app = root_router(state);

    app = app.layer(
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

    let mut origins: Vec<HeaderValue> = vec![];
    if let Some(frontend_urls) = config.frontend_urls {
        for frontend_url in frontend_urls {
            origins.push(frontend_url.parse().unwrap());
        }
    }
    if !origins.is_empty() {
        app = app.layer(
            CorsLayer::new()
                .allow_headers([
                    header::CONTENT_TYPE,
                    header::CONTENT_LENGTH,
                    header::CONTENT_LANGUAGE,
                ])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::HEAD,
                    Method::OPTIONS,
                ])
                .allow_credentials(true) // allow frontend to send us cookies
                .allow_origin(origins),
        );
    }

    app
}

fn root_router<S>(state: AppState) -> Router<S> {
    let app = Router::new();

    // the ServeDir path is relative to where the binary is run,
    // thus this is assuming working directory is `izakaya/tatami`
    let app = app.nest_service("/assets", ServeDir::new("./assets"));

    let app = app.route(
        "/favicon.ico",
        get(|| async { Redirect::permanent("/assets/favicon.ico") }),
    );
    let app = app.merge(endpoints::router(state.clone()));

    app.with_state(state)
}
