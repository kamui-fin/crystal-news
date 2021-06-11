use regex::RegexSet;
use sqlx::{postgres::PgPoolOptions, Pool};

use crate::api::error::ApiError;

pub async fn create_pool(db_url: String) -> Result<Pool<sqlx::Postgres>, ApiError> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| ApiError::DbError(e))
}

pub fn validate_password(passwd: &String) -> Result<(), validator::ValidationError> {
    let re_passwd = RegexSet::new(&[r"^[A-Za-z0-9]{6,}$", r"[A-Z]", r"[a-z]", r"[0-9]"])
        .expect("Invalid regex");
    let matched_len = re_passwd.matches(passwd).iter().count();

    if matched_len == re_passwd.len() {
        return Ok(());
    }

    let error = validator::ValidationError::new("password");
    Err(error)
}

pub mod jwt;
