use axum::{Router, routing::get};
use axum::response::{Html, Redirect};
use tower_http::services::ServeDir;

use tatami::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();

    let app = Router::new()
        // the path is relative to where the binary is run so under `ryokan/tatami` here
        .nest_service("/assets", ServeDir::new("./assets"))
        .route("/favicon.ico", get(|| async { Redirect::permanent("/assets/favicon.ico") }))
        .route("/", get(index));

    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();

    println!("Listening on {} 📢", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
