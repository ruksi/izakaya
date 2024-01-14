use axum::Extension;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::http::StatusCode;
use crate::auth::Visitor;

pub async fn require_login(
    Extension(visitor): Extension<Visitor>,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    if visitor.is_anonymous() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
    Ok(next.run(request).await)
}
