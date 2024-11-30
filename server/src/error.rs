use axum;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use dotenv;
use jsonwebtoken;
use openai::OpenAiError;
use serde_json::json;
use surrealdb;
use uuid;

#[derive(Debug)]
pub enum Error {
    Unauthorized,
    Forbidden(String),
    PaymentRequired,
    TooManyRequests,
    BadRequest(String),
    PayloadTooLarge,
    NotFound,
    Conflict(String),
    InternalServer,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            Error::Forbidden(error) => (StatusCode::FORBIDDEN, error),
            Error::PaymentRequired => {
                (StatusCode::PAYMENT_REQUIRED, "Payment required".to_string())
            }
            Error::TooManyRequests => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too many requests".to_string(),
            ),
            Error::BadRequest(error) => (StatusCode::BAD_REQUEST, error),
            Error::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            Error::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "Payload too large".to_string(),
            ),
            Error::Conflict(error) => (StatusCode::CONFLICT, error),
            Error::InternalServer => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl From<dotenv::Error> for Error {
    fn from(err: dotenv::Error) -> Self {
        tracing::error!("dotenv error: {}", err.to_string());
        Error::InternalServer
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error::BadRequest(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::BadRequest(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Error::BadRequest(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(err: bcrypt::BcryptError) -> Self {
        tracing::error!("fails to hash password, error: {}", err);
        Error::InternalServer
    }
}

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        tracing::error!("surrealdb error: {:?}", err);
        Error::InternalServer
    }
}

impl From<surrealdb::error::Db> for Error {
    fn from(err: surrealdb::error::Db) -> Self {
        tracing::error!("surrealdb db error: {:?}", err);
        Error::InternalServer
    }
}

impl From<OpenAiError> for Error {
    fn from(err: OpenAiError) -> Self {
        tracing::error!("openai error: {:?}", err);
        Error::InternalServer
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
