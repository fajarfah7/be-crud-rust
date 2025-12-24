use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use core::fmt;

#[derive(Serialize, Debug)]
pub struct ResponseErrorBody {
    status: u16,
    message: String,
    detail: Option<String>,
}

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
            ResponseError::BadRequest(msg) => (StatusCode::BAD_REQUEST, {
                let body = ResponseErrorBody {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: msg,
                    detail: None,
                };
                Json(body)
            }).into_response(),
            ResponseError::NotFound(msg) => (StatusCode::NOT_FOUND, {
                let body = ResponseErrorBody {
                    status: StatusCode::NOT_FOUND.as_u16(),
                    message: msg,
                    detail: None,
                };
                Json(body)
            }).into_response(),
            ResponseError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, {
                let body = ResponseErrorBody {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "internal server error".into(),
                    detail: Some("critical storage error".into()),
                };
                Json(body)
            }).into_response(),
            // ResponseError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response(),
        }
    }
}