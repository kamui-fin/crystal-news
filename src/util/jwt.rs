use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>,
}

impl Claims {
    pub fn new(sub: i32, exp_days: i64) -> Self {
        Claims {
            sub,
            iat: Utc::now(),
            exp: Utc::now() + chrono::Duration::days(exp_days),
        }
    }

    pub fn gen_token(&self, secret: &str) -> Result<String, crate::util::ApiError> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| ApiError::JwtError(e))
    }
}
