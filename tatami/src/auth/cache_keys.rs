use uuid::Uuid;

pub fn session_key<T: Into<String>>(token: T) -> String {
    // sessions are identified by the corresponding access token
    format!("tatami:session:{}", token.into())
}

pub fn access_token_from_session_key(session_key: &str) -> String {
    session_key
        .split(':')
        .nth(2)
        .unwrap_or_default()
        .to_string()
}

pub fn session_set_key(user_id: Uuid) -> String {
    format!("tatami:sessions:{}", user_id)
}
