use actix_web::{http::StatusCode, HttpResponse};
use argon2::Error as ArgonError;
use sqlx::Error as SqlxError;
use std::fmt;
pub type Result<T> = std::result::Result<T, ApiError>;
#[derive(Debug)]
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

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Invalid username or password")]
    InvalidCredentials,
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

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref msg) = self.message {
            write!(f, "{}: {}", self.code, msg)
        } else {
            write!(f, "{}", self.code)
        }
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

impl From<LoginError> for ApiError {
    fn from(error: LoginError) -> ApiError {
        match error {
            LoginError::InvalidCredentials => ApiError::new(StatusCode::BAD_REQUEST, None),
            LoginError::HashError(_) | LoginError::DatabaseError(_) => {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, None)
            }
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let mut res = HttpResponse::build(self.code);
        if let Some(ref msg) = self.message {
            return res.json(serde_json::json!({ "error": msg }));
        }
        res.into()
    }
}
