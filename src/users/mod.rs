pub mod claims_user;
mod postgres;
mod error;

pub use error::GetUserError;
pub use error::SetPasswordError;
use sqlx::Row;
use sqlx::postgres::PgRow;

use crate::data_stores::postgres::PostgresDatabase;

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

pub trait UserStore {
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError>;

    async fn set_password_hash(&self, id: i32, password: &str) -> Result<(), SetPasswordError>;
}

pub enum UserStoreOption {
    Postgres(PostgresDatabase),
}

impl UserStore for UserStoreOption {
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError> {
        match self {
            Self::Postgres(pg) => pg.get_by_username(username).await
        }
    }

    async fn set_password_hash(&self, id: i32, password: &str) -> Result<(), SetPasswordError> {
        match self {
            Self::Postgres(pg) => pg.set_password_hash(id, password).await
        }
    }
}

impl Default for UserStoreOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}