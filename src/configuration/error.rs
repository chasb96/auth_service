use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ConfigError(config::ConfigError);

impl Error for ConfigError { }

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error in configuration: {}", self.0)
    }
}

impl From<config::ConfigError> for ConfigError {
    fn from(value: config::ConfigError) -> Self {
        Self(value)
    }
}