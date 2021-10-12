use crate::error::{ApiError, ApiResult};
use actix_web::{dev, http::HeaderMap, FromRequest, HttpRequest};
use chrono::{prelude::*, Duration};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: i32,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt(pub String);

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

pub fn gen_token(claims: &Claims, secret: &str) -> Result<Jwt, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map(Jwt)
}

pub fn decode(token: Jwt, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<Claims>(
        &token.0,
        &DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|e| e.claims)
}

pub fn validate(token: Jwt, secret: &str) -> bool {
    decode(token, secret).is_ok()
}

pub fn get_jwt_from_bearer(headers: &HeaderMap) -> Option<Jwt> {
    if let Some(header) = headers.get("Authorization") {
        if let Ok(header_str) = header.to_str() {
            let bearer = header_str[6..].trim().to_string();
            return Some(Jwt(bearer));
        }
    }
    None
}

impl FromRequest for Jwt {
    type Error = ApiError;
    type Future = Ready<ApiResult<Jwt>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        if let Some(bearer) = get_jwt_from_bearer(req.headers()) {
            ok(bearer)
        } else {
            err(ApiError::InvalidToken)
        }
    }
}
