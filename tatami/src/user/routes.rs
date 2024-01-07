use axum::{Json, Router};
use axum::routing::get;

use crate::state::AppState;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(index))
        .with_state(state)
}

async fn index() -> Json<Vec<()>> {
    Json(vec![])
}
