use sqlx::{postgres::PgPoolOptions, Pool};

pub mod jwt;

pub async fn create_pool(db_url: String) -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
