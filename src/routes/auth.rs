use crate::models::refresh_token::RefreshToken;
use crate::{
    config::Context,
    models::user::{LoginCreds, SignUpCreds},
};
use crate::{error::Result, models::refresh_token::ReqRefresh};
use crate::{models::user::User, util::jwt::Claims};
use actix_web::{delete, post, web, HttpRequest, HttpResponse};
use sqlx::Pool;
use validator::Validate;

// TODO: Proper error handling and return messages

async fn generate_tokens(
    user_id: i32,
    secret: &String,
    pool: &Pool<sqlx::Postgres>,
) -> HttpResponse {
    let claims = Claims::new(user_id, 60);
    let acc_tk = claims.gen_token(secret);
    let ref_tk = RefreshToken::create(((60 * 60) * 24) * 100, user_id, pool).await;
    match ref_tk {
        Ok(ref_tk) => HttpResponse::Ok()
            .json(serde_json::json!({ "accessToken": acc_tk, "refreshToken": ref_tk })),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/signup")]
pub async fn signup(
    data: web::Json<SignUpCreds>,
    context: web::Data<Context>,
) -> Result<HttpResponse> {
    let user = User::register(&data, &context.pool).await?;
    Ok(generate_tokens(user.id, &context.config.jwt_secret, &context.pool).await)
}

#[post("/login")]
pub async fn login(
    data: web::Json<LoginCreds>,
    context: web::Data<Context>,
) -> Result<HttpResponse> {
    let user = User::login(&data, &context.pool).await?;
    Ok(generate_tokens(user.id, &context.config.jwt_secret, &context.pool).await)
}

#[delete("/logout")]
pub async fn logout(req: HttpRequest, context: web::Data<Context>) -> Result<HttpResponse> {
    let headers = req.headers();
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            let bearer = header_str[6..].trim().to_string();
            let claims = Claims::decode(bearer, &context.config.jwt_secret);
            info!("{:#?}", claims);
            if let Ok(claims) = claims {
                let user_id = claims.sub;
                RefreshToken::remove_all(user_id, &context.pool).await?;
                return Ok(HttpResponse::Ok().into());
            }
        }
    }
    Ok(HttpResponse::InternalServerError().into())
}

#[post("/refreshToken")]
pub async fn refresh_token(
    data: web::Json<ReqRefresh>,
    context: web::Data<Context>,
) -> Result<HttpResponse> {
    let ref_token = data.get_token(&context.pool).await?;
    if let Err(_) = ref_token.validate() {
        ref_token.remove(&context.pool).await?;
    }

    let claims = Claims::new(ref_token.user_id, 60);
    let acc_tk = claims.gen_token(&context.config.jwt_secret);
    Ok(HttpResponse::Ok()
        .json(serde_json::json!({ "accessToken": acc_tk, "refreshToken": ref_token.token })))
}
