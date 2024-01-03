use std::{error::Error, fmt::Display, sync::OnceLock};
use deadpool::managed::{Pool, BuildError};
use crate::configuration::Configuration;
use super::deadpool::ConnectionManager;

static CONNECTION_POOL: OnceLock<Pool<ConnectionManager>> = OnceLock::new();

#[derive(Debug)]
pub enum InitializeConnectionPoolError {
    Sqlx(sqlx::Error),
    Deadpool(BuildError),
}

impl Error for InitializeConnectionPoolError { }

impl Display for InitializeConnectionPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitializeConnectionPoolError::Sqlx(e) => write!(f, "InitializeConnectionPoolError::Sqlx({})", e),
            InitializeConnectionPoolError::Deadpool(e) => write!(f, "InitializeConnectionPoolError::Deadpool({})", e),
        }
    }
}

impl From<sqlx::Error> for InitializeConnectionPoolError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<BuildError> for InitializeConnectionPoolError {
    fn from(value: BuildError) -> Self {
        Self::Deadpool(value)
    }
}

#[derive(Debug)]
pub enum MigrateError {
    Sqlx(sqlx::migrate::MigrateError),
    Deadpool(deadpool::managed::PoolError<sqlx::error::Error>)
}

impl Error for MigrateError { }

impl Display for MigrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrateError::Sqlx(e) => write!(f, "MigrateError::Sqlx({})", e),
            MigrateError::Deadpool(e) => write!(f, "MigrateError::Deadpool({})", e),
        }
    }
}

impl From<sqlx::migrate::MigrateError> for MigrateError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<deadpool::managed::PoolError<sqlx::error::Error>> for MigrateError {
    fn from(value: deadpool::managed::PoolError<sqlx::error::Error>) -> Self {
        Self::Deadpool(value)
    }
}

pub struct PostgresDatabase {
    pub connection_pool: &'static Pool<ConnectionManager>,
}

impl Default for PostgresDatabase {
    fn default() -> Self {
        Self { 
            connection_pool: CONNECTION_POOL
                .get_or_init(|| {
                    let connection_string = Configuration::env()
                        .database
                        .connection_string
                        .to_string();

                    let manager = ConnectionManager {
                        connection_string,
                    };

                    Pool::builder(manager)
                        .build()
                        .expect("Unable to create connection pool")
                })
        }
    }
}