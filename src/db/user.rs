use crate::error::ApiError;
use argon2::Config as ArgonConfig;
use argon2::Variant::Argon2id;
use chrono::{DateTime, Utc};
use rand::RngCore;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use validator::{Validate, ValidationError};

use crate::error::ApiResult;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,20}$").expect("Invalid regex");
    static ref RE_PASS: Regex = Regex::new(r"^*{6,}$").expect("Invalid regex");
}

#[derive(Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Serialize, Deserialize)]
#[validate(schema(function = "validate_confirm_password", skip_on_field_errors = false))]
#[serde(rename_all = "camelCase")]
pub struct SignUpCreds {
    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(regex = "RE_PASS")]
    pub password: String,
    pub confirm_password: String,
}

#[derive(Validate, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCreds {
    #[validate(custom = "validate_email_or_username")]
    pub username_or_email: String,
    #[validate(regex = "RE_PASS")]
    pub password: String,
}

fn validate_confirm_password(data: &SignUpCreds) -> Result<(), ValidationError> {
    (data.password == data.confirm_password)
        .then(|| ())
        .ok_or_else(|| ValidationError::new("confirm_password"))
}

fn validate_email_or_username(email_usr: &String) -> Result<(), ValidationError> {
    let matches = validator::validate_email(email_usr) || RE_USERNAME.is_match(&email_usr);
    matches
        .then(|| ())
        .ok_or_else(|| ValidationError::new("username_email"))
}

fn hash_password(plain: &str, salt: &[u8]) -> Result<String, argon2::Error> {
    let config = ArgonConfig {
        variant: Argon2id,
        ..ArgonConfig::default()
    };
    argon2::hash_encoded(plain.as_bytes(), &salt, &config)
}

pub async fn register_user(creds: &SignUpCreds, pool: &Pool<sqlx::Postgres>) -> ApiResult<User> {
    if creds.validate().is_err() {
        return Err(ApiError::Validation);
    }

    let exist_user = sqlx::query!(
        "SELECT user_id FROM users WHERE username = $1 LIMIT 1",
        creds.username
    )
    .fetch_one(pool)
    .await;

    if exist_user.is_ok() {
        return Err(ApiError::InvalidCredentials);
    }

    let mut salt = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut salt);

    let hashed_passwd =
        hash_password(&creds.password, &salt).map_err(|_| ApiError::InternalServerError)?;

    sqlx::query_as!(
        User,
        "INSERT INTO users(username, email, password, salt) VALUES ($1, $2, $3, $4) RETURNING *",
        creds.username,
        creds.email,
        hashed_passwd,
        salt.to_vec()
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)
}

pub async fn login_user(creds: &LoginCreds, pool: &Pool<sqlx::Postgres>) -> ApiResult<User> {
    let exist_user: User = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 OR email = $1 LIMIT 1",
        creds.username_or_email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::error::Error::RowNotFound => ApiError::InvalidCredentials,
        _ => ApiError::InternalServerError,
    })?;

    let input_hash = hash_password(&creds.password, &exist_user.salt)
        .map_err(|_| ApiError::InternalServerError)?;

    if input_hash == exist_user.password {
        Ok(exist_user)
    } else {
        Err(ApiError::InvalidCredentials)
    }
}
