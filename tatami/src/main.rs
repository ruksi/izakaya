use tokio::net::TcpListener;
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

    let app = tatami::get_app(&config).await;
    tracing::debug!("Listening on {} 📢", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
