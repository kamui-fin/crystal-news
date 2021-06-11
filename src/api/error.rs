use argon2::Error as ArgonError;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Hash error")]
    HashError(ArgonError),
    #[error("Database error")]
    DbError(SqlxError),
    #[error("JWT error")]
    JwtError(jsonwebtoken::errors::Error),
    #[error("Configuration error")]
    ConfigError,
}
