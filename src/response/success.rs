use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::request::pagination::PaginationRequest;

#[derive(Serialize, Debug)]
struct ResponseSuccessBody<T> {
    message: String,
    http_code: u16,
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<PaginationMeta>,
}

#[derive(Serialize, Debug)]
struct PaginationMeta {
    page: u32,
    per_page: u32,
    total_data: u64,
    total_page: u32,
}

#[derive(Debug)]
pub enum ResponseSuccess<T> {
    Success(StatusCode, Option<T>),
}

impl<T> IntoResponse for ResponseSuccess<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            ResponseSuccess::Success(status, data) => (status, {
                let body: ResponseSuccessBody<T> = ResponseSuccessBody {
                    message: "success".into(),
                    http_code: status.as_u16(),
                    data: data,
                    meta: None,
                };
                Json(body)
            })
                .into_response(),
        }
    }
}

impl<T> ResponseSuccessBody<T>
where
    T: Serialize,
{
    pub fn success(status: StatusCode, data: Option<T>) -> impl IntoResponse {
        let body: ResponseSuccessBody<T> = ResponseSuccessBody {
            message: "success".into(),
            http_code: status.as_u16(),
            data: data,
            meta: None,
        };
        (status, Json(body))
    }

    pub fn paginated(
        page: u32,
        per_page: u32,
        total_data: u64,
        data: Option<T>,
    ) -> impl IntoResponse {
        let status = StatusCode::OK;

        let total_page = ((total_data + per_page as u64 - 1) / per_page as u64) as u32;

        let body: ResponseSuccessBody<T> = ResponseSuccessBody {
            message: "success".into(),
            http_code: status.as_u16(),
            data: data,
            meta: Some(PaginationMeta {
                page: page,
                per_page: per_page,
                total_data: total_data,
                total_page: total_page,
            }),
        };

        (status, Json(body))
    }
}
