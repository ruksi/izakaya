use axum::Router;

use crate::state::AppState;

mod emails;
mod sessions;
mod users;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/sessions", sessions::router(state.clone()))
        .nest("/users", users::router(state.clone()))
        .nest("/emails", emails::router(state.clone()))
        // everything under /api (defined above) requires authentication (defined below)
        .route_layer(axum::middleware::from_fn(
            crate::auth::middleware::require_login,
        ))
}
