use serde::Deserialize;

mod error;
mod env;

#[derive(Deserialize)]
pub struct Configuration {
    pub database: DatabaseConfig,
    pub authentication: AuthenticationConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
}

#[derive(Deserialize)]
pub struct AuthenticationConfig {
    pub token: String,
    pub hmac_key: String,
}