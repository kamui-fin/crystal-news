use crate::error::{ApiError, ApiResult};
use chrono::{DateTime, FixedOffset, Utc};
use serde::Serialize;
use sqlx::{Pool, Postgres};

#[derive(Debug, Serialize, Clone)]
pub struct Article {
    article_id: i32,
    source_id: i32,
    item_link: Option<String>,
    pub title: Option<String>,
    description: Option<String>,
    author: Option<String>,
    pub pub_date: Option<DateTime<Utc>>,
    content: Option<String>,
    pub guid: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AddArticle {
    pub source_id: i32,
    pub item_link: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub pub_date: Option<DateTime<FixedOffset>>,
    pub content: Option<String>,
    pub guid: Option<String>,
}

pub async fn add_articles(articles: &[AddArticle], pool: &Pool<Postgres>) -> ApiResult<()> {
    for article in articles {
        sqlx::query!(
        "INSERT INTO articles(source_id, item_link, title, description, author, pub_date, content, guid)
                    VALUES($1,$2,$3,$4,$5,$6, $7, $8)",
        article.source_id,
        article.item_link,
        article.title,
        article.description,
        article.author,
        article.pub_date,
        article.content,
        article.guid,
    )
        .execute(pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;
    }
    Ok(())
}

pub async fn get_latest_article(pool: &Pool<Postgres>) -> Option<Article> {
    sqlx::query_as!(
        Article,
        "SELECT * FROM articles ORDER BY pub_date DESC LIMIT 1"
    )
    .fetch_one(pool)
    .await
    .ok()
}
