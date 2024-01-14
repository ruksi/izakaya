use uuid::Uuid;

pub fn access_token_key<T: Into<String>>(token: T) -> String {
    format!("tatami:access-token:{}", token.into())
}

pub fn access_token_list_key(user_id: Uuid) -> String {
    format!("tatami:access-tokens:{}", user_id)
}
