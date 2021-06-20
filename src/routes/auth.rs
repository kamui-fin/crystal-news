use crate::{
    config::Context,
    db::{
        refresh_token::{delete_refresh_token, get_token_from_req, RespToken},
        user::{login_user, register_user, LoginCreds, SignUpCreds},
    },
};
use crate::{db::refresh_token::create_refresh_token, util::get_bearer};
use crate::{db::refresh_token::ReqRefresh, error::ApiResult};
use crate::{
    error::{ApiError, Error},
    util::jwt::{gen_token, Claims},
};
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use validator::Validate;

async fn generate_tokens(
    user_id: i32,
    ref_token: Option<String>,
    context: &web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let claims = Claims::new(user_id, context.config.acc_token_exp);
    let acc_tk = gen_token(&claims, &context.config.jwt_secret);
    let ref_tk = match ref_token {
        Some(ref_token) => ref_token,
        None => create_refresh_token(context.config.ref_token_exp, user_id, &context.pool).await?,
    };

    match acc_tk {
        Ok(acc_tk) => Ok(HttpResponse::Ok().json(RespToken {
            access_token: acc_tk,
            refresh_token: ref_tk,
        })),
        Err(_) => Err(Error::ApiResponse(ApiError::code(
            StatusCode::INTERNAL_SERVER_ERROR,
        ))),
    }
}

pub async fn signup(
    data: web::Json<SignUpCreds>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user = register_user(&data, &context.pool).await?;
    generate_tokens(user.user_id, None, &context).await
}

pub async fn login(
    data: web::Json<LoginCreds>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let user = login_user(&data, &context.pool).await?;
    generate_tokens(user.user_id, None, &context).await
}

pub async fn logout(req: HttpRequest, context: web::Data<Context>) -> ApiResult<HttpResponse> {
    let headers = req.headers();
    let bearer = get_bearer(headers);

    if let Some(bearer) = bearer {
        let req_refresh = ReqRefresh { token: bearer };
        let token = get_token_from_req(&req_refresh, &context.pool).await?;
        delete_refresh_token(&token, &context.pool).await?;

        return Ok(HttpResponse::Ok().into());
    }

    Err(Error::ApiResponse(ApiError::code(
        StatusCode::INTERNAL_SERVER_ERROR,
    )))
}

pub async fn refresh_token(
    data: web::Json<ReqRefresh>,
    context: web::Data<Context>,
) -> ApiResult<HttpResponse> {
    let ref_token = get_token_from_req(&data, &context.pool).await?;
    if let Err(_) = ref_token.validate() {
        delete_refresh_token(&ref_token, &context.pool).await?;
    }
    Ok(generate_tokens(ref_token.user_id, Some(ref_token.token), &context).await?)
}
