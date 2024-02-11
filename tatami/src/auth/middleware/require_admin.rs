use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;

use crate::auth::CurrentUser;
use crate::prelude::*;

pub async fn require_admin(
    Extension(current_user): Extension<CurrentUser>,
    request: Request,
    next: Next,
) -> Result<Response> {
    if !current_user.is_superuser {
        return Err(Error::Forbidden);
    }
    Ok(next.run(request).await)
}
