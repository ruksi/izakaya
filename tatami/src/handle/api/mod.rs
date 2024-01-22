use axum::Router;

use crate::state::AppState;

mod sessions;
mod settings;
mod users;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/sessions", sessions::router(state.clone()))
        .nest("/settings", settings::router(state.clone()))
        .nest("/users", users::router(state.clone()))
        // everything under /api requires authentication (defined above)
        .route_layer(axum::middleware::from_fn(crate::auth::require_login))
}
