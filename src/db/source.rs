use crate::error::ApiResult;
use chrono::{DateTime, Utc};
use serde::Serialize;
use validator::Validate;

#[derive(Debug)]
pub struct Source {
    source_id: i32,
    user_id: i32,
    title: String,
    description: String,
    website: String,
    feed_link: String,
    last_updated: DateTime<Utc>,
}

#[derive(Validate, Serialize)]
pub struct AddSource {
    pub user_id: i32,
    #[validate(url)]
    pub feed_link: String,
    #[validate(length(min = 1))]
    pub title: String,
}
