use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use validator::Validate;

use crate::{
    error::{ApiError, ApiResult, Error},
    feed,
};

#[derive(Debug)]
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
    pub user_id: i32,
    #[validate(url)]
    pub feed_link: String,
    #[validate(length(min = 1))]
    pub title: String,
}

pub async fn subscribe_source(
    add_source: AddSource,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<Source> {
    add_source
        .validate()
        .map_err(|e| crate::error::Error::Validation(e))?;
    let info = feed::get_source_info(&add_source.feed_link)
        .await
        .ok_or(Error::ApiResponse(ApiError::message(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not fetch rss info",
        )))?;
    sqlx::query_as!(
        Source,
        "INSERT INTO sources(user_id, title, description, website, feed_link, last_updated)
                    VALUES($1,$2,$3,$4,$5,$6) RETURNING *",
        add_source.user_id,
        add_source.title,
        info.description,
        info.link,
        add_source.feed_link,
        info.last_build_date
            .map(|t| DateTime::parse_from_rfc2822(t.as_ref()).unwrap())
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::Db(e))
}
