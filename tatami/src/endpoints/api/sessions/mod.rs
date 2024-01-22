use axum::routing::{delete, get, post};
use axum::Router;

use create::create;
use destroy::destroy;
use list::list;

use crate::state::AppState;

mod create;
mod destroy;
mod list;

#[cfg(test)]
mod tests;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        // TODO: delete route should work with "session id" but we don't have that yet?
        .route("/", get(list))
        .route("/", post(create))
        .route("/", delete(destroy))
        .with_state(state)
}
