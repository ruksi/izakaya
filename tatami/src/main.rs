use axum::http::{HeaderValue, Method};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // populate environment variables from `.env` file without overriding
    dotenvy::dotenv().ok();

    let config = tatami::config::Config::load();

    tracing_subscriber::registry()
        .with(EnvFilter::builder().parse(&config.rust_log).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind(config.bind_address())
        .await
        .unwrap();

    let mut app = tatami::get_app(&config).await;

    let mut origins: Vec<HeaderValue> = vec![];
    if let Some(frontend_url) = config.frontend_url {
        origins.push(frontend_url.parse().unwrap());
    }
    if !origins.is_empty() {
        app = app.layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::HEAD,
                    Method::OPTIONS,
                ])
                .allow_credentials(true)
                .allow_origin(origins)
        );
    }

    // todo: https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware

    tracing::debug!("Listening on {} 📢", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
