use uuid::Uuid;

pub use amend::*;
pub use create::*;
pub use describe::*;
pub use destroy::*;
pub use list::*;

mod amend;
mod create;
mod list;
mod describe;
mod destroy;

#[cfg(test)]
mod tests;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
}
