use axum::extract::State;

use crate::state::AppState;

pub async fn panicz(State(_state): State<AppState>) {
    panic!("oh no!")
}

#[cfg(test)]
mod tests {
    use crate::test_utils::mock_server;

    #[should_panic(expected = "oh no!")]
    #[sqlx::test]
    async fn works(db: sqlx::PgPool) {
        let server = mock_server(&db).await;
        server.get("/panicz").await;
    }
}
