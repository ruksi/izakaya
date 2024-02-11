use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
}
