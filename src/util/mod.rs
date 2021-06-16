use actix_web::http::HeaderMap;
use sqlx::{postgres::PgPoolOptions, Pool};

pub mod jwt;

pub async fn create_pool(db_url: String) -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

pub fn get_bearer(headers: &HeaderMap) -> Option<String> {
    if let Some(header) = headers.get("Authorization") {
        if let Ok(header_str) = header.to_str() {
            let bearer = header_str[6..].trim().to_string();
            return Some(bearer);
        }
    }
    None
}
