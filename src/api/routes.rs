use crate::models::user::CreateUser;
use crate::util::jwt::Claims;
use actix_web::{post, web, HttpResponse};

use super::error::ApiError;

#[post("/signup")]
pub async fn signup(
    data: web::Json<CreateUser>,
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
