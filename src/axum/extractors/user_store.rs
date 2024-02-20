use std::ops::Deref;
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::{StatusCode, request::Parts}};
use crate::users::UserStoreOption;

pub struct UserStoreExtractor(UserStoreOption);

impl Deref for UserStoreExtractor {
    type Target = UserStoreOption;

    fn deref(&self) -> &Self::Target {
        &self.0
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