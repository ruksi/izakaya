use axum::Router;

use crate::state::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/users", crate::user::route::router(state.clone()))
        // everything under API requires authentication
        .route_layer(axum::middleware::from_fn(crate::auth::require_login))
        // ... but session management is partially open to allow login
        .nest("/me", crate::me::route::router(state))
}
