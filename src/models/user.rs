use crate::error::{
    LoginError,
    SignUpError::{self, RequirementError, UsernameAlreadyExists},
};
use argon2::Config as ArgonConfig;
use argon2::Variant::Argon2id;
use chrono::{DateTime, Utc};
use rand::RngCore;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use validator::{Validate, ValidationError};
use sqlx::Pool;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,20}$").expect("Invalid regex");
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Deserialize)]
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

#[derive(Validate, Deserialize)]
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

impl User {
    fn hash_password(plain: &String, salt: &[u8]) -> Result<String, argon2::Error> {
        let config = ArgonConfig {
            variant: Argon2id,
            mem_cost: 15728,
            ..ArgonConfig::default()
        };
        argon2::hash_encoded(plain.as_bytes(), &salt, &config)
    }

    pub async fn register(
        creds: &SignUpCreds,
        pool: &Pool<sqlx::Postgres>,
    ) -> Result<Self, SignUpError> {
        if let Err(e) = creds.validate() {
            return Err(RequirementError(e));
        }

        let exist_user = sqlx::query!(
            "SELECT id FROM users WHERE username = $1 LIMIT 1",
            creds.username
        )
        .fetch_one(pool)
        .await;

        if let Ok(_) = exist_user {
            return Err(UsernameAlreadyExists);
        }

        let mut salt = [0u8; 8];
        rand::thread_rng().fill_bytes(&mut salt);

        let hashed_passwd =
            User::hash_password(&creds.password, &salt).map_err(|e| SignUpError::HashError(e))?;

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
        .map_err(|e| SignUpError::DatabaseError(e))
    }

    pub async fn login(
        creds: &LoginCreds,
        pool: &Pool<sqlx::Postgres>,
    ) -> Result<Self, LoginError> {
        let exist_user: User = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1 OR email = $1 LIMIT 1",
            creds.username_or_email
        )
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => LoginError::InvalidCredentials,
            e => LoginError::DatabaseError(e),
        })?;

        let input_hash = User::hash_password(&creds.password, &exist_user.salt)
            .map_err(|e| LoginError::HashError(e))?;

        if input_hash == exist_user.password {
            Ok(exist_user)
        } else {
            Err(LoginError::InvalidCredentials)
        }
    }
}
