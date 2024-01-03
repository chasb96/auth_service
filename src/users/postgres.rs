use crate::data_stores::postgres::PostgresDatabase;
use super::{UserStore, User};

impl UserStore for PostgresDatabase {
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, super::error::GetUserError> {
        const GET_BY_USERNAME_QUERY: &'static str = r#"
            SELECT id, username, password_hash
            FROM users
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(GET_BY_USERNAME_QUERY)
            .bind(username)
            .map(User::from)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(Into::into)
    }

    async fn set_password_hash(&self, id: i32, password_hash: &str) -> Result<(), super::SetPasswordError> {
        const SET_PASSWORD_HASH_QUERY: &'static str = r#"
            UPDATE users
            SET password_hash = $2
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(SET_PASSWORD_HASH_QUERY)
            .bind(id)
            .bind(password_hash)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}