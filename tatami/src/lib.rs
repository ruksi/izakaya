use std::time::Duration;

use axum::extract::Request;
use axum::http::{header, HeaderName, HeaderValue, Method};
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub use crate::config::Config;
use crate::state::AppState;

mod auth;
mod config;
mod endpoints;
mod error;
mod prelude;
mod scripts;
mod session;
mod state;
mod user;
mod valid;

#[cfg(test)]
mod test_utils;

pub async fn run_server() {
    let config = Config::load();

    let registry = tracing_subscriber::registry();
    let registry = registry.with(EnvFilter::builder().parse(&config.rust_log).unwrap());
    let registry = registry.with(tracing_subscriber::fmt::layer());
    if !std::env::var("SENTRY_DSN").unwrap_or_default().is_empty() {
        // default Sentry settings:
        // * `tracing::info!` and up are collected as breadcrumbs
        // * `tracing::error!` are sent as error events
        // * `tracing::info_span!` and up are sent as transactions
        //
        // NB: a separate `tower-http` integration turns requests to Sentry transactions
        let registry = registry.with(sentry::integrations::tracing::layer());
        registry.init();
    } else {
        registry.init();
    }

    let listener = TcpListener::bind(config.bind_address()).await.unwrap();
    let app = get_app(config).await;

    tracing::debug!("Listening on {} 📢", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

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

    let state = AppState::new(config.clone(), db_pool, cache_pool);
    let mut app = root_router(state);

    app = app.layer(
        TraceLayer::new_for_http()
            .make_span_with(|req: &Request| {
                let method = req.method();
                let uri = req.uri();
                tracing::debug_span!("request", %method, %uri)
            })
            .on_failure(()), // we `trace::error!()` ourselves, we don't want all 404 to be errors
    );

    if !std::env::var("SENTRY_DSN").unwrap_or_default().is_empty() {
        // record all requests as Sentry transactions filtered by sampling rate
        let sentry_layer = tower::ServiceBuilder::new()
            .layer(sentry::integrations::tower::NewSentryLayer::<Request>::new_from_top())
            .layer(sentry::integrations::tower::SentryHttpLayer::with_transaction());
        app = app.layer(sentry_layer);
    }

    match config.frontend_urls.len() {
        0 => tracing::warn!("CORS disabled, no FRONTEND_URL set"),
        _ => tracing::debug!("CORS enabled for {:?}", config.frontend_urls),
    }
    let mut allowed_origins: Vec<HeaderValue> = vec![];
    for frontend_url in config.frontend_urls {
        allowed_origins.push(frontend_url.parse().unwrap());
    }
    if !allowed_origins.is_empty() {
        app = app.layer(
            CorsLayer::new()
                .allow_headers([
                    header::CONTENT_TYPE,
                    header::CONTENT_LENGTH,
                    header::CONTENT_LANGUAGE,
                    HeaderName::from_static("csrf-token"),
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
                .allow_origin(allowed_origins),
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
