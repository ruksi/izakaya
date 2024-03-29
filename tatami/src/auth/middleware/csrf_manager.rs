use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;

use crate::auth::csrf::is_valid_csrf_token;
use crate::auth::{cookie, Visitor};
use crate::prelude::*;
use crate::state::AppState;

pub async fn csrf_manager(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
    cookies: tower_cookies::Cookies,
    request: Request,
    next: Next,
) -> Result<Response> {
    if !state.config.csrf_enabled {
        return Ok(next.run(request).await);
    }

    // TODO: skip CSRF manager if authenticated with a Bearer token 🐻

    // if the request is not "safe" (GET | HEAD | OPTIONS | TRACE),
    // require a CSRF token
    if !request.method().is_safe() {
        let Some(csrf_cookie) = cookies.get(cookie::CSRF_TOKEN) else {
            tracing::debug!("CSRF cookie missing");
            // also return them a new, valid CSRF token
            cookies.add(cookie::bake_csrf(&state.config, visitor.session_id));
            return Err(Error::Unauthorized);
        };
        let csrf_cookie_token = csrf_cookie.value();

        let csrf_header_token = request
            .headers()
            .get("CSRF-Token")
            .and_then(|value| value.to_str().ok());
        let Some(csrf_header_token) = csrf_header_token else {
            tracing::debug!("CSRF header missing");
            return Err(Error::Unauthorized);
        };

        if csrf_cookie_token != csrf_header_token {
            tracing::debug!("CSRF token mismatch");

            if !is_valid_csrf_token(
                &state.config.csrf_secret,
                visitor.session_id,
                csrf_cookie_token,
            ) {
                // also, the CSRF token in the cookie was wrong
                // so return them a new, valid CSRF cookie token
                cookies.add(cookie::bake_csrf(&state.config, visitor.session_id));
            }

            return Err(Error::Unauthorized);
        }

        if !is_valid_csrf_token(
            &state.config.csrf_secret,
            visitor.session_id,
            csrf_cookie_token,
        ) {
            tracing::debug!("CSRF token invalid");
            // also return them a new, valid CSRF token
            cookies.add(cookie::bake_csrf(&state.config, visitor.session_id));
            return Err(Error::Unauthorized);
        }
    }

    // we _could_ move this after "next" but the auth state (`visitor`) might be incorrect
    if cookies.get(cookie::CSRF_TOKEN).is_none() {
        cookies.add(cookie::bake_csrf(&state.config, visitor.session_id));
    }

    Ok(next.run(request).await)
}
