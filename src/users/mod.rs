mod claims_user;
mod postgres;
mod error;

pub use error::GetUserError;
pub use error::SetPasswordError;
use sqlx::Row;
use sqlx::postgres::PgRow;

pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl From<PgRow> for User {
    fn from(row: PgRow) -> Self {
        User {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
        }
    }
}

pub struct ClaimsUser {
    pub id: i32,
    pub username: String,
}

pub trait UserStore {
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError>;

    async fn set_password_hash(&self, id: i32, password: &str) -> Result<(), SetPasswordError>;
}