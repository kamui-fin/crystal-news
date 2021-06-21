use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Article {
    article_id: i32,
    source_id: i32,
    item_link: Option<String>,
    title: Option<String>,
    description: Option<String>,
    author: Option<String>,
    content: Option<String>,
}
