use crate::{
    config::Context,
    db::source::{add_source, add_subscription, get_all_sources, AddSource},
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
    let res = add_source(data.0, &context.pool).await?;
    add_subscription(user_id, res.source_id, &context.pool).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_all_feeds(jwt: JWT, context: web::Data<Context>) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    let res = get_all_sources(user_id, &context.pool).await?;
    Ok(HttpResponse::Ok().json(res))
}
