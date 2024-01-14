#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::pool::Pool<sqlx::Postgres>,
    pub cache_pool: deadpool_redis::Pool,
    pub cookie_secret: axum_extra::extract::cookie::Key,
}
