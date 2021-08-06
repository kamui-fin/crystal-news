use crate::{
    error::{ApiError, ApiResult},
    feed,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use validator::Validate;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub source_id: i32,
    user_id: i32,
    title: String,
    description: String,
    website: String,
    pub feed_link: String,
    last_updated: Option<DateTime<Utc>>,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct AddSource {
    #[validate(url)]
    pub feed_link: String,
    #[validate(length(min = 1))]
    pub title: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateSource {
    #[validate(length(min = 1))]
    pub title: String,
}

pub async fn add_source(
    user_id: i32,
    add_source: AddSource,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<Source> {
    add_source.validate().map_err(|_| ApiError::Validation)?;
    let info = feed::get_channel_info(&add_source.feed_link)
        .await
        .ok_or(ApiError::RssFetch)?;
    sqlx::query_as!(
        Source,
        "INSERT INTO sources(user_id, title, description, website, feed_link, last_updated)
                    VALUES($1,$2,$3,$4,$5,$6) RETURNING *",
        user_id,
        add_source.title,
        info.description,
        info.link,
        add_source.feed_link,
        info.last_build_date
            .map(|t| DateTime::parse_from_rfc2822(t.as_ref()).unwrap())
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
}

pub async fn update_source(
    user_id: i32,
    source_id: i32,
    update_source: UpdateSource,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<Source> {
    update_source.validate().map_err(|_| ApiError::Validation)?;
    sqlx::query_as!(
        Source,
        "UPDATE sources SET title = $1 WHERE user_id = $2 AND source_id = $3 RETURNING *",
        update_source.title,
        user_id,
        source_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
}

pub async fn delete_source(
    user_id: i32,
    source_id: i32,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<()> {
    sqlx::query!(
        "DELETE FROM sources WHERE user_id = $1 AND source_id = $2",
        user_id,
        source_id
    )
    .execute(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
    .map(|_| ())
}

pub async fn get_source(
    user_id: i32,
    source_id: i32,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<Source> {
    sqlx::query_as!(
        Source,
        "SELECT * FROM sources WHERE user_id = $1 AND source_id = $2",
        user_id,
        source_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
}

pub async fn get_all_sources(user_id: i32, pool: &Pool<sqlx::Postgres>) -> ApiResult<Vec<Source>> {
    sqlx::query_as!(Source, "SELECT * FROM sources WHERE user_id = $1", user_id)
        .fetch_all(pool)
        .await
        .map_err(|_| ApiError::InternalServerError)
}
