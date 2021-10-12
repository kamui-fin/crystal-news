use crate::db::refresh_token::create_refresh_token;
use crate::error::ApiResult;
use crate::{
    config::Context,
    db::{
        refresh_token::{delete_refresh_token, get_token_from_req},
        user::{login_user, register_user, LoginCreds, SignUpCreds},
    },
};
use crate::{
    error::ApiError,
    jwt::{gen_token, Claims},
};
use actix_web::{cookie::Cookie, web, HttpMessage, HttpRequest, HttpResponse};
use time::Duration;
use validator::Validate;

async fn generate_tokens(user_id: i32, context: &web::Data<Context>) -> ApiResult<HttpResponse> {
    let claims = Claims::new(user_id, context.config.acc_token_exp);
    let acc_tk = gen_token(&claims, &context.config.jwt_secret);
    let ref_tk = create_refresh_token(context.config.ref_token_exp, user_id, &context.pool).await?;

    match acc_tk {
        Ok(acc_tk) => Ok(HttpResponse::Ok()
            .cookie(
                Cookie::build("refresh_token", &ref_tk)
                    .max_age(Duration::seconds(context.config.ref_token_exp))
                    .http_only(true)
                    .secure(false)
                    .path("/")
                    .finish(),
            )
            .json(serde_json::json!({
               "token": acc_tk.0,
               "tokenExpiry": context.config.acc_token_exp,
               "refreshToken": ref_tk,
               "refreshTokenExpiry": context.config.ref_token_exp,
                "userId": user_id
            }))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

pub async fn signup(
    data: web::Json<SignUpCreds>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user = register_user(&data, &context.pool).await?;
    generate_tokens(user.user_id, &context).await
}

pub async fn login(
    data: web::Json<LoginCreds>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user = login_user(&data, &context.pool).await?;
    generate_tokens(user.user_id, &context).await
}

// pub async fn logout(req: HttpRequest, context: web::Data<Context>) -> ApiResult<HttpResponse> {
//     let headers = req.headers();
//     let bearer = get_jwt_from_bearer(headers);
//
//     if let Some(bearer) = bearer {
//         let req_refresh = ReqRefresh { token: bearer };
//         let token = get_token_from_req(&req_refresh, &context.pool).await?;
//         delete_refresh_token(&token, &context.pool).await?;
//
//         return Ok(HttpResponse::Ok().into());
//     }
//
//     Err(ApiError::InternalServerError)
// }
//
pub async fn refresh_token(
    req: HttpRequest,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let cookie_token = req.cookie("refresh_token");
    if let Some(token) = cookie_token {
        let ref_token = get_token_from_req(token.value(), &context.pool).await?;
        if ref_token.validate().is_ok() {
            delete_refresh_token(&ref_token, &context.pool).await?;
            return generate_tokens(ref_token.user_id, &context).await;
        }
    }
    Err(ApiError::InvalidToken)
}
