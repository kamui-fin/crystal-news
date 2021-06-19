use chrono::{prelude::*, Duration};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: i32,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn new(sub: i32, exp_sec: i64) -> Self {
        let now = Utc::now();
        let exp_time = now + Duration::seconds(exp_sec);
        Claims {
            sub,
            iat: now.timestamp(),
            exp: exp_time.timestamp(),
        }
    }
}

pub fn gen_token(claims: &Claims, secret: &str) -> Result<String, Error> {
    jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode(token: String, secret: &str) -> Result<Claims, Error> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|e| e.claims)
}

pub fn validate(token: String, secret: &str) -> bool {
    decode(token, secret).is_ok()
}
