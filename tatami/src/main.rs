use axum::{Router, routing::get};
use axum::response::{Html, Redirect};
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

    let app = Router::new()
        // the path is relative to where the binary is run so under `ryokan/tatami` here
        .nest_service("/assets", ServeDir::new("./assets"))
        .route("/favicon.ico", get(|| async { Redirect::permanent("/assets/favicon.ico") }))
        .route("/", get(index));

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
