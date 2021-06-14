use crate::{
    config::Context,
    models::user::{LoginCreds, SignUpCreds},
};
use crate::{models::user::User, util::jwt::Claims};
use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::error::Result;

fn send_token(user_id: i32, secret: &String) -> HttpResponse {
    let claims = Claims::new(user_id, 180);
    let token = claims.gen_token(secret);
    HttpResponse::Ok().json(serde_json::json!({ "token": token }))
}

#[post("/signup")]
pub async fn signup(
    data: web::Json<SignUpCreds>,
    context: web::Data<Context>,
) -> Result<HttpResponse> {
    let user = User::register(&data, &context.pool).await?;
    Ok(send_token(user.id, &context.config.jwt_secret))
}

#[post("/login")]
pub async fn login(
    data: web::Json<LoginCreds>,
    context: web::Data<Context>,
) -> Result<HttpResponse> {
    let user = User::login(&data, &context.pool).await?;
    Ok(send_token(user.id, &context.config.jwt_secret))
}

#[post("/logout")]
pub async fn logout(req: HttpRequest, context: web::Data<Context>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("OK"))
}
