use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{header, request::Parts};

use crate::app_middleware::jwt_token::jwt::verify_token;
use crate::app_response::error::ResponseError;

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ResponseError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(ResponseError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(ResponseError::Unauthorized)?;

        let claims = verify_token(token).map_err(|_| ResponseError::InvalidToken)?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
