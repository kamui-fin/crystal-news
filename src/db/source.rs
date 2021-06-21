use crate::{
    error::{ApiError, ApiResult},
    feed,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct Source {
    source_id: i32,
    user_id: i32,
    title: String,
    description: String,
    website: String,
    feed_link: String,
    last_updated: Option<DateTime<Utc>>,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct AddSource {
    #[validate(url)]
    pub feed_link: String,
    #[validate(length(min = 1))]
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteSource {
    pub source_id: i32,
}

pub async fn add_source(
    user_id: i32,
    add_source: AddSource,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<Source> {
    add_source.validate().map_err(|_| ApiError::Validation)?;
    let info = feed::get_source_info(&add_source.feed_link)
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

pub async fn delete_source(
    user_id: i32,
    del_source: DeleteSource,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<()> {
    sqlx::query!(
        "DELETE FROM sources WHERE user_id = $1 AND source_id = $2",
        user_id,
        del_source.source_id
    )
    .execute(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
    .map(|_| ())
}
