use crate::{
    config::Context,
    db::source::{add_source, delete_source, AddSource, DeleteSource},
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

pub async fn unsubscribe_feed(
    jwt: JWT,
    data: web::Json<DeleteSource>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user_id = decode(jwt, &context.config.jwt_secret).unwrap().sub;
    delete_source(user_id, data.0, &context.pool).await?;
    Ok(HttpResponse::Ok().body(""))
}
