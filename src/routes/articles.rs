use crate::{
    config::Context,
    db::{
        article::{add_articles, get_latest_article},
        source::get_all_sources,
    },
    error::ApiResult,
    feed::generate_feed,
    jwt::{decode, JWT},
};
use actix_web::{web, HttpResponse};

pub async fn get_all_feed(jwt: JWT, context: web::Data<Context>) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    let sources = get_all_sources(user_id, &context.pool).await?;
    let feed = generate_feed(sources).await.unwrap_or_default();
    let latest_db = get_latest_article(&context.pool).await;
    let newest_articles;
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
    add_articles(&newest_articles, &context.pool).await?;
    Ok(HttpResponse::Ok().json(feed))
}
