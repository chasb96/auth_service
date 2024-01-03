use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{StatusCode, request::Parts}};
use crate::configuration::Configuration;

pub struct ConfigurationExtractor(&'static Configuration);

#[async_trait]
impl<T> FromRequestParts<T> for ConfigurationExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(ConfigurationExtractor(Configuration::env()))
    }
}

impl Deref for ConfigurationExtractor {
    type Target = Configuration;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}