use axum::routing::{delete, get, post};
use axum::Router;

use create::create;
use destroy::destroy;
use list::list;
use log_in::log_in;
use log_out::log_out;
use sign_up::sign_up;
use verify::verify;

use crate::state::AppState;

mod create;
mod destroy;
mod list;
mod log_in;
mod log_out;
mod sign_up;
mod verify;

#[cfg(test)]
mod tests;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        // TODO: delete route should work with "session id" but we don't have that yet?
        .route("/", get(list))
        .route("/", post(create))
        .route("/", delete(destroy))
        .route("/sign-up", post(sign_up))
        .route("/log-in", post(log_in))
        .route("/verify", get(verify))
        .route("/log-out", post(log_out))
        .with_state(state)
}
