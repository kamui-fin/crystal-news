use actix_web::{dev::Body, http::StatusCode, HttpResponse};
use argon2::Error as ArgonError;
use sqlx::Error as SqlxError;
use std::fmt;

pub struct ApiError {
    code: StatusCode,
    message: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum SignUpError {
    #[error("Credentials failed to meet requirement")]
    RequirementError(validator::ValidationErrors),
    #[error("Username already exists")]
    UsernameAlreadyExists,
    #[error("Something went wrong")]
    DatabaseError(SqlxError),
    #[error("Failed to process password")]
    HashError(ArgonError),
}

impl ApiError {
    pub fn new(code: StatusCode, message: Option<String>) -> Self {
        Self { code, message }
    }
}

impl From<SignUpError> for ApiError {
    fn from(error: SignUpError) -> ApiError {
        match error {
            SignUpError::RequirementError(_) | SignUpError::UsernameAlreadyExists => {
                ApiError::new(StatusCode::BAD_REQUEST, None)
            }
            SignUpError::HashError(_) | SignUpError::DatabaseError(_) => {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, None)
            }
        }
    }
}
impl Into<HttpResponse> for ApiError {
    fn into(self) -> HttpResponse {
        let res = HttpResponse::new(self.code);
        if let Some(msg) = self.message {
            return res.set_body(Body::Message(Box::new(msg)));
        }
        res
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref msg) = self.message {
            write!(f, "{}: {}", self.code, msg)
        } else {
            write!(f, "{}", self.code)
        }
    }
}
