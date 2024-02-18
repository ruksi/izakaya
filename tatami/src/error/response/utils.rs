use super::ErrorOut;
use axum::Json;
use std::fmt::Display;

pub fn error_message<T: Display>(message: T) -> Json<ErrorOut> {
    Json(ErrorOut::new(message.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn json_message_works() {
        let json = error_message("hello");
        assert_eq!(json.message, "hello");
        assert_eq!(json.issues, None);
    }
}
