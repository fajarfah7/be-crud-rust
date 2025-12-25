use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{app_middleware::jwt_token::jwt::generate_token, app_response::{error::ResponseError, success::ResponseSuccess}, login::handler::types::Login};

static USERNAME: &str = "admin";
static PASSWORD: &str = "password";
static USER_ID: &str = "super_admin";

pub async fn login(
    Json(req): Json<Login>
) -> Result<impl IntoResponse, ResponseError> {
    if req.username != USERNAME || req.password != PASSWORD {
        return Err(ResponseError::BadRequest("invalid username or password".into()))
    }

    let generate_token = generate_token(USER_ID);
    let token = match generate_token {
        Ok(token) => token,
        Err(_) => {
            return Err(ResponseError::InternalServerError)
        }
    };

    Ok(ResponseSuccess::Object(StatusCode::OK, Some(token)))
}