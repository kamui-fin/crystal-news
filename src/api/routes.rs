use crate::models::user::CreateUser;
use crate::util::jwt::Claims;
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse};

use super::error::ApiError;

#[post("/signup")]
pub async fn signup(
    data: web::Json<CreateUser>,
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<HttpResponse, HttpResponse> {
    let user = data
        .register(pool.as_ref())
        .await
        .map_err(|e| -> HttpResponse { ApiError::from(e).into() })?;
    let claims = Claims::new(user.id, 180);
    let token = claims
        .gen_token("secret string")
        .map_err(|_| -> HttpResponse {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, None).into()
        })?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}
