use axum::routing::get;
use axum::Router;

use create::create;
use destroy::destroy;
use list::list;

use crate::state::AppState;

mod create;
mod destroy;
mod list;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        // TODO: delete route should work with "session id" but we don't have that yet?
        .route("/", get(list).post(create).delete(destroy))
        .with_state(state)
}
