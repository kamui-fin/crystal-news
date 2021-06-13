use crate::error::{
    LoginError,
    SignUpError::{self, RequirementError, UsernameAlreadyExists},
};
use argon2::Config as ArgonConfig;
use argon2::Variant::Argon2id;
use rand::RngCore;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use validator::{Validate, ValidationError};

use sqlx::{types::time::OffsetDateTime, Pool};

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,20}$").expect("Invalid regex");
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: Vec<u8>,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Validate, Deserialize)]
pub struct UserCredentials {
    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(custom = "User::validate_password")]
    pub password: String,
}

impl User {
    fn hash_password(plain: &String, salt: &[u8]) -> Result<String, argon2::Error> {
        let config = ArgonConfig {
            variant: Argon2id,
            ..ArgonConfig::default()
        };
        argon2::hash_encoded(plain.as_bytes(), &salt, &config)
    }

    pub fn validate_password(passwd: &String) -> Result<(), ValidationError> {
        let re_passwd = RegexSet::new(&[r"^[A-Za-z0-9]{6,}$", r"[A-Z]", r"[a-z]", r"[0-9]"])
            .expect("Invalid regex");
        let matched_len = re_passwd.matches(passwd).iter().count();
        if matched_len == re_passwd.len() {
            return Ok(());
        }
        let error = ValidationError::new("password");
        Err(error)
    }

    pub async fn register(
        creds: &UserCredentials,
        pool: &Pool<sqlx::Postgres>,
    ) -> Result<Self, SignUpError> {
        let exist_user = sqlx::query!(
            "SELECT id FROM users WHERE username = $1 LIMIT 1",
            creds.username
        )
        .fetch_one(pool)
        .await;

        if let Err(e) = creds.validate() {
            return Err(RequirementError(e));
        }

        if let Ok(_) = exist_user {
            return Err(UsernameAlreadyExists);
        }

        let mut salt = [0u8; 8];
        rand::thread_rng().fill_bytes(&mut salt);

        let hashed_passwd =
            User::hash_password(&creds.password, &salt).map_err(|e| SignUpError::HashError(e))?;

        sqlx::query_as!(
            User,
            "INSERT INTO users(username, password, salt) VALUES ($1, $2, $3) RETURNING *",
            creds.username,
            hashed_passwd,
            salt.to_vec()
        )
        .fetch_one(pool)
        .await
        .map_err(|e| SignUpError::DatabaseError(e))
    }

    pub async fn login(
        creds: &UserCredentials,
        pool: &Pool<sqlx::Postgres>,
    ) -> Result<Self, LoginError> {
        let exist_user: User = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1 LIMIT 1",
            creds.username
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
