use sqlx::FromRow;
use uuid::Uuid;

pub use amend::*;
pub use create::*;
pub use describe::*;
pub use destroy::*;
pub use list::*;

mod amend;
mod create;
mod describe;
mod destroy;
mod list;

#[derive(FromRow, Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
}
