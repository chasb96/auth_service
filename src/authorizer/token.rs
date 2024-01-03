use std::sync::OnceLock;
use crate::configuration::Configuration;
use super::Authorizer;

static TOKEN: OnceLock<String> = OnceLock::new();

pub struct TokenAuthorizer {
    token: &'static str
}

impl Default for TokenAuthorizer {
    fn default() -> Self {
        Self { 
            token: TOKEN
                .get_or_init(|| {
                    Configuration::env()
                        .authentication
                        .token
                        .clone()
                })
        }
    }
}

impl Authorizer for TokenAuthorizer {
    async fn verify<'a>(&self, token: &'a str) -> bool {
        self.token == token
    }
}