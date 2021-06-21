use actix_web::{
    dev::HttpResponseBuilder,
    http::{header, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};
use jsonwebtoken::errors::Error as JwtError;
use serde_json::json;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Internal server error")]
    InternalServerError,
    #[display(fmt = "Bad request")]
    BadRequest,
    #[display(fmt = "Expired token")]
    TokenExpired,
    #[display(fmt = "Invalid token")]
    InvalidToken,
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials,
    #[display(fmt = "Invalid input")]
    Validation,
    #[display(fmt = "Could not fetch RSS information")]
    RssFetch,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(json!({"error": self.to_string()}))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalServerError | ApiError::RssFetch => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest | ApiError::Validation => StatusCode::BAD_REQUEST,
            ApiError::TokenExpired | ApiError::InvalidToken | ApiError::InvalidCredentials => {
                StatusCode::UNAUTHORIZED
            }
        }
    }
}

impl From<JwtError> for ApiError {
    fn from(e: JwtError) -> ApiError {
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => ApiError::InvalidToken,
            _ => ApiError::InvalidCredentials,
        }
    }
}
