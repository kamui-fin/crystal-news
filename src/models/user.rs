use crate::api::error::ApiError;
use crate::util::validate_password;
use argon2::Config as ArgonConfig;
use rand::RngCore;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

use sqlx::{types::time::OffsetDateTime, Pool};

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,20}$").expect("Invalid regex");
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: Vec<u8>,
    pub created_at: OffsetDateTime,
}

#[derive(Validate, Deserialize)]
pub struct CreateUser {
    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

impl CreateUser {
    pub async fn register(&self, pool: &Pool<sqlx::Postgres>) -> Result<User, ApiError> {
        let exist_user = sqlx::query!(
            "SELECT id FROM users WHERE username = $1 LIMIT 1",
            self.username
        )
        .fetch_one(pool)
        .await;

        if let Err(_) = self.validate() {
            panic!("todo");
        }

        if let Ok(_) = exist_user {
            panic!("todo");
        }

        let mut salt = [0u8; 8];
        rand::thread_rng().fill_bytes(&mut salt);

        let config = ArgonConfig {
            variant: argon2::Variant::Argon2id,
            ..ArgonConfig::default()
        };

        let hashed_passwd = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ApiError::HashError(e))?;

        sqlx::query_as!(
            User,
            "INSERT INTO users(username, password, salt) VALUES ($1, $2, $3) RETURNING *",
            self.username,
            hashed_passwd,
            salt.to_vec(),
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::DbError(e))
    }
}
