use chrono::{prelude::*, Duration};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

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
            exp: Utc::now() + Duration::days(exp_days),
        }
    }

    pub fn gen_token(&self, secret: &str) -> String {
        jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap() // will not panic since we're using a constructor
    }
}
