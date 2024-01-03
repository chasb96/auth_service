use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::{StatusCode, request::Parts}};

use crate::{data_stores::postgres::PostgresDatabase, users::{UserStore, User, GetUserError, SetPasswordError}};

pub struct UserStoreExtractor(PostgresDatabase);

impl UserStore for UserStoreExtractor {
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError> {
        self.0.get_by_username(username).await
    }

    async fn set_password_hash(&self, id: i32, password: &str) -> Result<(), SetPasswordError> {
        self.0.set_password_hash(id, password).await
    }
}

impl Default for UserStoreExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for UserStoreExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(UserStoreExtractor::default())
    }
}