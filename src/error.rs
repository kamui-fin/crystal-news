use std::fmt;

use actix_web::{dev::HttpResponseBuilder, http::StatusCode, HttpResponse};
use serde_json::json;

pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ApiResponse(ApiError),
    Db(sqlx::Error),
    Validation(validator::ValidationErrors),
    Hash(argon2::Error),
}

#[derive(Debug)]
pub struct ApiError {
    code: StatusCode,
    message: ApiErrorResponse,
}

#[derive(Debug)]
pub enum ApiErrorResponse {
    Message(&'static str),
    None,
}

impl ApiError {
    pub fn message(code: StatusCode, message: &'static str) -> Self {
        Self {
            code,
            message: ApiErrorResponse::Message(message),
        }
    }

    pub fn code(code: StatusCode) -> Self {
        Self {
            code,
            message: ApiErrorResponse::None,
        }
    }

    pub fn repr(&self) -> serde_json::Value {
        match self.message {
            ApiErrorResponse::Message(m) => json!({ "error": m }),
            ApiErrorResponse::None => json!({}),
        }
    }
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ApiResponse(err) => err.code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Error::ApiResponse(err) => HttpResponseBuilder::new(err.code).json(err.repr()),
            _ => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .json(json!({"error": "An unexpected error has occured"})),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ApiResponse(e) => write!(f, "{}", e.repr()),
            Error::Db(e) => e.fmt(f),
            Error::Validation(e) => e.fmt(f),
            Error::Hash(e) => e.fmt(f),
        }
    }
}
