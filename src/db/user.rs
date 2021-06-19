use crate::error::{ApiError, Error};
use actix_web::http::StatusCode;
use argon2::Config as ArgonConfig;
use argon2::Variant::Argon2id;
use chrono::{DateTime, Utc};
use rand::RngCore;
use regex::{Regex, RegexSet};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use validator::{Validate, ValidationError};

use crate::error::ApiResult;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,20}$").expect("Invalid regex");
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
pub struct SignUpCreds {
    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_password")]
    pub password: String,
    pub confirm_password: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct LoginCreds {
    #[validate(custom = "validate_email_or_username")]
    pub username_or_email: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

fn validate_confirm_password(data: &SignUpCreds) -> Result<(), ValidationError> {
    (data.password == data.confirm_password)
        .then(|| ())
        .ok_or(ValidationError::new("confirm_password"))
}

fn validate_email_or_username(email_usr: &String) -> Result<(), ValidationError> {
    let matches = validator::validate_email(email_usr) || RE_USERNAME.is_match(&email_usr);
    matches
        .then(|| ())
        .ok_or(ValidationError::new("username_email"))
}

fn validate_password(passwd: &String) -> Result<(), ValidationError> {
    let re_passwd = RegexSet::new(&[r"^[A-Za-z0-9]{6,}$", r"[A-Z]", r"[a-z]", r"[0-9]"])
        .expect("Invalid regex");
    let matched_len = re_passwd.matches(passwd).iter().count();
    if matched_len == re_passwd.len() {
        return Ok(());
    }
    let error = ValidationError::new("password");
    Err(error)
}

fn hash_password(plain: &String, salt: &[u8]) -> Result<String, argon2::Error> {
    let config = ArgonConfig {
        variant: Argon2id,
        ..ArgonConfig::default()
    };
    argon2::hash_encoded(plain.as_bytes(), &salt, &config)
}

pub async fn register_user(creds: &SignUpCreds, pool: &Pool<sqlx::Postgres>) -> ApiResult<User> {
    if let Err(e) = creds.validate() {
        return Err(Error::Validation(e));
    }

    let exist_user = sqlx::query!(
        "SELECT user_id FROM users WHERE username = $1 LIMIT 1",
        creds.username
    )
    .fetch_one(pool)
    .await;

    if let Ok(_) = exist_user {
        return Err(Error::ApiResponse(ApiError::message(
            StatusCode::BAD_REQUEST,
            "Username already exists",
        )));
    }

    let mut salt = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut salt);

    let hashed_passwd = hash_password(&creds.password, &salt).map_err(|e| Error::Hash(e))?;

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
    .map_err(|e| Error::Db(e))
}

pub async fn login_user(creds: &LoginCreds, pool: &Pool<sqlx::Postgres>) -> ApiResult<User> {
    let invalid_creds = || {
        Error::ApiResponse(ApiError::message(
            StatusCode::BAD_REQUEST,
            "Invalid credentials",
        ))
    };
    let exist_user: User = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 OR email = $1 LIMIT 1",
        creds.username_or_email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::error::Error::RowNotFound => invalid_creds(),
        e => Error::Db(e),
    })?;

    let input_hash =
        hash_password(&creds.password, &exist_user.salt).map_err(|e| Error::Hash(e))?;

    if input_hash == exist_user.password {
        Ok(exist_user)
    } else {
        Err(invalid_creds())
    }
}
