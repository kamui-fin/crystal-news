use crate::error::ApiError;
use actix_web::http::StatusCode;
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

#[derive(Deserialize)]
pub struct ReqRefresh {
    pub token: String,
}

impl ReqRefresh {
    pub async fn get_token(
        &self,
        pool: &sqlx::Pool<Postgres>,
    ) -> crate::error::Result<RefreshToken> {
        sqlx::query_as!(
            RefreshToken,
            "SELECT * FROM refresh_token WHERE token = $1",
            self.token
        )
        .fetch_one(pool)
        .await
        .map_err(|_| ApiError::new(StatusCode::NO_CONTENT, None))
    }
}

fn validate_exp(exp: &DateTime<Utc>) -> Result<(), validator::ValidationError> {
    (exp < &chrono::Utc::now())
        .then(|| ())
        .ok_or(ValidationError::new("expired"))
}

impl RefreshToken {
    pub async fn create(
        exp_sec: i64,
        user_id: i32,
        pool: &Pool<Postgres>,
    ) -> Result<String, sqlx::Error> {
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
    }

    pub async fn remove(&self, pool: &Pool<Postgres>) -> crate::error::Result<()> {
        sqlx::query!("DELETE FROM refresh_token WHERE tk_id = $1", self.tk_id)
            .execute(pool)
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, None))
            .map(|_| ())
    }

    pub async fn remove_all(uid: i32, pool: &Pool<Postgres>) -> crate::error::Result<()> {
        sqlx::query!("DELETE FROM refresh_token WHERE user_id = $1", uid)
            .execute(pool)
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, None))
            .map(|_| ())
    }
}
