use crate::{
    config::Context,
    db::source::{subscribe_source, AddSource},
    error::ApiResult,
};
use actix_web::{web, HttpResponse};
use log::info;

pub async fn subscribe_feed(
    data: web::Json<AddSource>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let res = subscribe_source(data.0, &context.pool).await?;
    info!("{:#?}", res);
    Ok(HttpResponse::Ok().body(""))
}
