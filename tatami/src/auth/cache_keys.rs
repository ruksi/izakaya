use uuid::Uuid;

pub fn session_key<T: Into<String>>(token: T) -> String {
    // sessions are identified by the corresponding access token
    format!("tatami:session:{}", token.into())
}

pub fn session_list_key(user_id: Uuid) -> String {
    format!("tatami:sessions:{}", user_id)
}
