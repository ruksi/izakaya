use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

use self::healthz::healthz;
use self::index::index;
use self::log_in::log_in;
use self::log_out::log_out;
use self::sign_up::sign_up;
use self::verify::verify;

pub mod api;
mod healthz;
mod index;
mod log_in;
mod log_out;
mod sign_up;
mod verify;

#[cfg(test)]
mod tests;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        // TODO: delete route should work with "session id" but we don't have that yet?
        .route("/", get(index))
        .route("/healthz", get(healthz))
        .route("/sign-up", post(sign_up))
        .route("/log-in", post(log_in))
        .route("/log-out", post(log_out))
        .route("/verify", get(verify))
        .nest("/api", api::router(state.clone()))
        .with_state(state)
}
