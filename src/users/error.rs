use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum GetUserError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for GetUserError { }

impl Display for GetUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetUserError::Sqlx(e) => write!(f, "Error running query: {}", e),
            GetUserError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<SqlxError> for GetUserError {
    fn from(value: SqlxError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<PoolError<SqlxError>> for GetUserError {
    fn from(value: PoolError<SqlxError>) -> Self {
        Self::Pool(value)
    }
}

#[derive(Debug)]
pub enum SetPasswordError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for SetPasswordError { }

impl Display for SetPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetPasswordError::Sqlx(e) => write!(f, "Error running query: {}", e),
            SetPasswordError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<SqlxError> for SetPasswordError {
    fn from(value: SqlxError) -> Self {
        Self::Sqlx(value)
    }
}

impl From<PoolError<SqlxError>> for SetPasswordError {
    fn from(value: PoolError<SqlxError>) -> Self {
        Self::Pool(value)
    }
}