use crate::{
    config::Context,
    db::source::{
        add_source, delete_source, get_all_sources, get_source, update_source, AddSource,
        UpdateSource,
    },
    error::ApiResult,
    jwt::{decode, JWT},
};
use actix_web::{web, HttpResponse};

pub async fn subscribe_feed(
    jwt: JWT,
    data: web::Json<AddSource>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    let res = add_source(user_id, data.0, &context.pool).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update_feed(
    jwt: JWT,
    feed_id: web::Path<i32>,
    data: web::Json<UpdateSource>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    let res = update_source(user_id, feed_id.0, data.0, &context.pool).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn unsubscribe_feed(
    jwt: JWT,
    feed_id: web::Path<i32>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    delete_source(user_id, feed_id.0, &context.pool).await?;
    Ok(HttpResponse::Ok().body(""))
}

pub async fn get_feed(
    jwt: JWT,
    feed_id: web::Path<i32>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    let res = get_source(user_id, feed_id.0, &context.pool).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_all_feeds(jwt: JWT, context: web::Data<Context>) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    let res = get_all_sources(user_id, &context.pool).await?;
    Ok(HttpResponse::Ok().json(res))
}
