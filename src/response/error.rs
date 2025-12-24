use axum::{http::StatusCode, response::{IntoResponse, Response}};
use core::fmt;

#[derive(Debug)]
pub enum ResponseError {
    BadRequest(String),
    NotFound(String),
    DatabaseError,
    // InternalServerError,
}
impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::BadRequest(msg) => write!(f, "warning_bad_request: {}", msg),
            ResponseError::NotFound(msg) => write!(f, "warning_not_found: {}", msg),
            ResponseError::DatabaseError => write!(f, "error_storage"),
            // ResponseError::InternalServerError => write!(f, "error_server"),
        }
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            ResponseError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            ResponseError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            ResponseError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "storage error").into_response(),
            // ResponseError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response(),
        }
    }
}