use crate::{
    config::Context,
    models::user::{LoginCreds, SignUpCreds},
};
use crate::{error::ApiResult, models::refresh_token::ReqRefresh};
use crate::{models::refresh_token::RefreshToken, util::get_bearer};
use crate::{models::user::User, util::jwt::Claims};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::Pool;
use validator::Validate;

async fn generate_tokens(
    user_id: i32,
    secret: &String,
    ref_token: Option<String>,
    pool: &Pool<sqlx::Postgres>,
) -> ApiResult<HttpResponse> {
    let claims = Claims::new(user_id, 60);
    let acc_tk = claims.gen_token(secret);
    let ref_tk = match ref_token {
        Some(ref_token) => ref_token,
        None => RefreshToken::create(((60 * 60) * 24) * 100, user_id, pool).await?,
    };
    Ok(HttpResponse::Ok()
        .json(serde_json::json!({ "accessToken": acc_tk, "refreshToken": ref_tk })))
}

pub async fn signup(
    data: web::Json<SignUpCreds>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user = User::register(&data, &context.pool).await?;
    Ok(generate_tokens(user.id, &context.config.jwt_secret, None, &context.pool).await?)
}

pub async fn login(
    data: web::Json<LoginCreds>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user = User::login(&data, &context.pool).await?;
    Ok(generate_tokens(user.id, &context.config.jwt_secret, None, &context.pool).await?)
}

pub async fn logout(req: HttpRequest, context: web::Data<Context>) -> ApiResult<HttpResponse> {
    let headers = req.headers();
    let bearer = get_bearer(headers);
    if let Some(bearer) = bearer {
        let req_refresh = ReqRefresh {
            token: bearer
        };

        RefreshToken::from_req(&req_refresh, &context.pool)
            .await?
            .remove(&context.pool)
            .await?;

        return Ok(HttpResponse::Ok().into());
    }
    Ok(HttpResponse::InternalServerError().into())
}

pub async fn refresh_token(
    data: web::Json<ReqRefresh>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let ref_token = RefreshToken::from_req(&data, &context.pool).await?;
    if let Err(_) = ref_token.validate() {
        ref_token.remove(&context.pool).await?;
    }
    Ok(generate_tokens(
        ref_token.user_id,
        &context.config.jwt_secret,
        Some(ref_token.token),
        &context.pool,
    )
    .await?)
}
