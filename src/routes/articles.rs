use crate::{
    config::Context,
    db::{
        article::{add_articles, get_latest_article, AddArticle, Article},
        source::{get_all_sources, get_source, Source},
    },
    error::ApiResult,
    feed::generate_feed,
    jwt::{decode, Jwt},
};
use actix_web::{web, HttpResponse};

pub async fn update_feed(
    sources: Vec<Source>,
    context: web::Data<Context>,
) -> Option<Vec<AddArticle>> {
    let latest_db = get_latest_article(&context.pool).await;
    let newest_articles;
    let feed = generate_feed(sources).await.unwrap_or_default();
    if let Some(latest_db) = latest_db {
        let idx = feed
            .iter()
            .position(|f| {
                if latest_db.guid.is_none() {
                    return f.title == latest_db.title;
                }
                f.guid == latest_db.guid
            })
            .unwrap_or(feed.len());

        newest_articles = feed[..idx].to_vec();
    } else {
        newest_articles = feed.to_vec();
    }
    if add_articles(&newest_articles, &context.pool).await.is_err() {
        return None;
    };
    return Some(feed);
}

pub async fn get_all_feed(jwt: Jwt, context: web::Data<Context>) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    let sources = get_all_sources(user_id, &context.pool).await?;
    let feed = update_feed(sources, context).await;
    Ok(HttpResponse::Ok().json(feed))
}

pub async fn get_feed_by_source(
    source_id: web::Path<i32>,
    jwt: Jwt,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    let source = get_source(*source_id, &context.pool).await?;
    let feed = update_feed(vec![source], context).await;
    Ok(HttpResponse::Ok().json(feed))
}
