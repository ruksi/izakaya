use axum::routing::{get, post};
use axum::Router;
use tower_cookies::CookieManagerLayer;

use crate::state::AppState;

use self::healthz::healthz;
use self::index::index;
use self::log_in::log_in;
use self::log_out::log_out;
use self::panicz::panicz;
use self::sign_up::sign_up;
use self::verify::verify;

pub mod api;
mod healthz;
mod index;
mod log_in;
mod log_out;
mod panicz;
mod sign_up;
mod verify;

#[cfg(test)]
mod tests;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(index))
        .route("/healthz", get(healthz))
        .route("/panicz", get(panicz))
        .route("/sign-up", post(sign_up))
        .route("/log-in", post(log_in))
        .route("/log-out", post(log_out))
        .route("/verify", get(verify))
        .nest("/api", api::router(state.clone()))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::record_visit,
        ))
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
