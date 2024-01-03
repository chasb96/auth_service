use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{StatusCode, request::Parts}};
use crate::authorizer::TokenAuthorizer;

pub struct AuthorizerExtractor(TokenAuthorizer);

#[async_trait]
impl<T> FromRequestParts<T> for AuthorizerExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(AuthorizerExtractor(TokenAuthorizer::default()))
    }
}

impl Deref for AuthorizerExtractor {
    type Target = TokenAuthorizer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}