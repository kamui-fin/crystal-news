use crate::error::RefreshTokenError;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshToken {
    tk_id: i32,
    pub token: String,
    pub user_id: i32,
    #[validate(custom = "validate_exp")]
    expiration: DateTime<Utc>,
}

fn validate_exp(exp: &DateTime<Utc>) -> Result<(), validator::ValidationError> {
    (exp < &chrono::Utc::now())
        .then(|| ())
        .ok_or(ValidationError::new("expired"))
}

#[derive(Deserialize)]
pub struct ReqRefresh {
    pub token: String,
}

impl RefreshToken {
    pub async fn from_req(
        req_ref: &ReqRefresh,
        pool: &sqlx::Pool<Postgres>,
    ) -> Result<Self, RefreshTokenError> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM refresh_token WHERE token = $1",
            req_ref.token
        )
        .fetch_one(pool)
        .await
        .map_err(|e| RefreshTokenError::DatabaseError(e))
    }

    pub async fn create(
        exp_sec: i64,
        user_id: i32,
        pool: &Pool<Postgres>,
    ) -> Result<String, RefreshTokenError> {
        let mut buffer = Uuid::encode_buffer();
        let token = Uuid::new_v4().to_simple().encode_upper(&mut buffer);
        let exp = chrono::Utc::now() + chrono::Duration::seconds(exp_sec);
        sqlx::query_as!(
            Self,
            "INSERT INTO refresh_token(token, user_id, expiration) VALUES ($1, $2, $3) RETURNING *",
            &*token,
            user_id,
            exp
        )
        .fetch_one(pool)
        .await
        .map(|t| t.token)
        .map_err(|e| RefreshTokenError::DatabaseError(e))
    }

    pub async fn remove(&self, pool: &Pool<Postgres>) -> Result<(), RefreshTokenError> {
        sqlx::query!("DELETE FROM refresh_token WHERE tk_id = $1", self.tk_id)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| RefreshTokenError::DatabaseError(e))
    }
}
