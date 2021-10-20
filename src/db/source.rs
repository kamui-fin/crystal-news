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
}

pub struct Subscription {
    sub_id: i32,
    source_id: i32,
    user_id: i32,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateSource {
    #[validate(length(min = 1))]
    pub title: String,
}

pub async fn add_source(add_source: AddSource, pool: &Pool<sqlx::Postgres>) -> ApiResult<Source> {
    add_source.validate().map_err(|_| ApiError::Validation)?;
    let info = feed::get_channel_info(&add_source.feed_link)
        .await
        .ok_or(ApiError::RssFetch)?;

    let exists = sqlx::query_as!(
        Source,
        "SELECT * FROM sources WHERE feed_link = $1",
        add_source.feed_link
    )
    .fetch_one(pool)
    .await
    .ok();

    match exists {
        None => {
            let record = sqlx::query_as!(
                Source,
                "INSERT INTO sources(title, description, website, feed_link, last_updated)
                    VALUES($1,$2,$3,$4,$5) RETURNING *",
                info.title,
                info.description,
                info.link,
                add_source.feed_link,
                info.last_build_date
                    .map(|t| DateTime::parse_from_rfc2822(t.as_ref()).unwrap())
            )
            .fetch_one(pool)
            .await
            .map_err(|_| ApiError::InternalServerError);

            return record;
        }
        Some(exists) => {
            return Ok(exists);
        }
    }
}

pub async fn add_subscription(
    user_id: i32,
    source_id: i32,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<()> {
    sqlx::query!(
        "INSERT INTO user_subscriptions(source_id, user_id) VALUES($1,$2)",
        source_id,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
    .map(|_| ())
}

pub async fn delete_source(source_id: i32, pool: &Pool<sqlx::Postgres>) -> ApiResult<()> {
    sqlx::query!("DELETE FROM sources WHERE source_id = $1", source_id)
        .execute(pool)
        .await
        .map_err(|_| ApiError::InternalServerError)
        .map(|_| ())
}

pub async fn delete_subscription(sub_id: i32, pool: &Pool<sqlx::Postgres>) -> ApiResult<()> {
    sqlx::query!("DELETE FROM user_subscriptions WHERE sub_id = $1", sub_id)
        .execute(pool)
        .await
        .map_err(|_| ApiError::InternalServerError)
        .map(|_| ())
}

pub async fn get_source(source_id: i32, pool: &Pool<sqlx::Postgres>) -> ApiResult<Source> {
    sqlx::query_as!(
        Source,
        "SELECT * FROM sources WHERE source_id = $1",
        source_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
}

pub async fn get_all_sources(user_id: i32, pool: &Pool<sqlx::Postgres>) -> ApiResult<Vec<Source>> {
    sqlx::query_as!(Source, "SELECT * FROM sources WHERE source_id IN (SELECT source_id FROM user_subscriptions WHERE user_id = $1)", user_id)
        .fetch_all(pool)
        .await
        .map_err(|_| ApiError::InternalServerError)
}
