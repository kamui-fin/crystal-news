use crate::error::{ApiError, ApiResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshToken {
    token_id: i32,
    pub token: String,
    pub user_id: i32,
    #[validate(custom = "validate_exp")]
    expiration: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RespToken {
    pub access_token: String,
    pub refresh_token: String,
}

fn validate_exp(exp: &DateTime<Utc>) -> Result<(), validator::ValidationError> {
    (exp > &chrono::Utc::now())
        .then(|| ())
        .ok_or_else(|| ValidationError::new("expired"))
}

pub async fn get_token_from_req(
    req_ref: &str,
    pool: &sqlx::Pool<Postgres>,
) -> ApiResult<RefreshToken> {
    sqlx::query_as!(
        RefreshToken,
        "SELECT * FROM refresh_token WHERE token = $1",
        req_ref
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
}

pub async fn create_refresh_token(
    exp_sec: i64,
    user_id: i32,
    pool: &Pool<Postgres>,
) -> ApiResult<String> {
    let mut buffer = Uuid::encode_buffer();
    let token = Uuid::new_v4().to_simple().encode_upper(&mut buffer);
    let exp = chrono::Utc::now() + chrono::Duration::seconds(exp_sec);
    sqlx::query_as!(
        RefreshToken,
        "INSERT INTO refresh_token(token, user_id, expiration) VALUES ($1, $2, $3) RETURNING *",
        &*token,
        user_id,
        exp
    )
    .fetch_one(pool)
    .await
    .map(|t| t.token)
    .map_err(|_| ApiError::InternalServerError)
}

pub async fn delete_refresh_token(
    ref_token: &RefreshToken,
    pool: &Pool<Postgres>,
) -> ApiResult<()> {
    sqlx::query!(
        "DELETE FROM refresh_token WHERE token_id = $1",
        ref_token.token_id
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|_| ApiError::InternalServerError)
}
