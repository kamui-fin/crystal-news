use crate::util::jwt::Claims;
use crate::{models::user::UserCredentials, util::Context};
use actix_web::{post, web, HttpResponse};

use super::error::Result;

fn send_token(user_id: i32, secret: &String) -> HttpResponse {
    let claims = Claims::new(user_id, 180);
    let token = claims.gen_token(secret);
    HttpResponse::Ok().json(serde_json::json!({ "token": token }))
}

#[post("/signup")]
pub async fn signup(
    data: web::Json<UserCredentials>,
    context: web::Data<Context>,
) -> Result<HttpResponse> {
    let user = data.register(&context.pool).await?;
    Ok(send_token(user.id, &context.config.jwt_secret))
}

#[post("/login")]
pub async fn login(
    data: web::Json<UserCredentials>,
    context: web::Data<Context>,
) -> Result<HttpResponse> {
    let user = data.login(&context.pool).await?;
    Ok(send_token(user.id, &context.config.jwt_secret))
}
