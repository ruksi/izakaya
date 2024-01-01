use std::time::Duration;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;

use crate::config::Config;

pub mod config;
mod error;

pub async fn get_app(config: &Config) -> Router {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Can't connect to database");

    let app = Router::new();

    // the ServeDir path is relative to where the binary is run,
    // thus this is assuming working directory is `ryokan/tatami`
    let app = app.nest_service("/assets", ServeDir::new("./assets"));

    let app = app.route("/", get(index));
    let app = app.route("/favicon.ico", get(|| async { Redirect::permanent("/assets/favicon.ico") }));
    let app = app.route("/healthz", get(healthz));

    let app = app.with_state(pool);

    app
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn healthz(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("SELECT 'OK'")
        .fetch_one(&pool)
        .await
        .map_err(error::internal)
}
