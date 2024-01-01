use std::time::Duration;

use axum::{Router, routing::get};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use tatami::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "tatami=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Can't connect to database");

    let app = Router::new()
        // the path is relative to where the binary is run so under `ryokan/tatami` here
        .nest_service("/assets", ServeDir::new("./assets"))
        .route("/favicon.ico", get(|| async { Redirect::permanent("/assets/favicon.ico") }))
        .route("/healthz", get(healthz))
        .route("/", get(index))
        .with_state(pool);

    let address = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(address)
        .await
        .unwrap();

    tracing::debug!("Listening on {} 📢", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
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
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
    where E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
