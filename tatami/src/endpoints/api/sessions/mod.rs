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
        .route("/", get(list))
        .route("/", post(create))
        .route("/:access_token_prefix", delete(destroy))
        .with_state(state)
}
