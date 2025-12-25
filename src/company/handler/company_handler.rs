use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{company::handler::types::ProcessCompanyRequest, app_request::{pagination::PaginationRequest, path_uuid::PathUuid}};
use crate::company::{
    handler::map_company_error::map_usecase_company_error,
    repository::company_repository::CompanyRepository,
};
use crate::company::{
    handler::map_company_error::validate_company_input, usecase::company_usecase::CompanyUsecase,
};
use crate::app_response::error::ResponseError;
use crate::app_response::success::ResponseSuccess;

// order parameter in handler MUST
// 1. STATE
// 2. PATH
// 3. QUERY
// 4. HEADER / EXTENSION
// 5. JSON / FORM / MULTIPART

pub async fn create_company_handler<R: CompanyRepository>(
    State(usecase): State<Arc<CompanyUsecase<R>>>,
    Json(req): Json<ProcessCompanyRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    validate_company_input(&req)?;

    let company = usecase
        .create_company(req.name, req.email, req.code, req.phone_number, req.address)
        .await
        .map_err(map_usecase_company_error)?;

    Ok(ResponseSuccess::Object(StatusCode::CREATED, Some(company)))
}

pub async fn update_company_handler<R: CompanyRepository>(
    State(usecase): State<Arc<CompanyUsecase<R>>>,
    PathUuid(id): PathUuid,
    Json(req): Json<ProcessCompanyRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    validate_company_input(&req)?;

    let company = usecase
        .update_company(
            id,
            req.name,
            req.email,
            req.code,
            req.phone_number,
            req.address,
        )
        .await
        .map_err(map_usecase_company_error)?;

    Ok(ResponseSuccess::Object(StatusCode::CREATED, Some(company)))
}

pub async fn delete_company_handler<R: CompanyRepository>(
    State(usecase): State<Arc<CompanyUsecase<R>>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ResponseError> {
    usecase
        .delete_company(id)
        .await
        .map_err(map_usecase_company_error)?;

    Ok(ResponseSuccess::NoData::<()>(StatusCode::OK))
}

pub async fn get_companies_handler<R: CompanyRepository>(
    State(usecase): State<Arc<CompanyUsecase<R>>>,
    Query(q): Query<PaginationRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(1);
    let search = q.search.unwrap_or("".into());
    let sort = q.sort.unwrap_or("".into());
    
    let query = PaginationRequest {
        page: Some(page),
        per_page: Some(per_page),
        offset: Some((page - 1) * per_page),
        search: Some(search),
        sort: Some(sort),
    };
    let company_list_data = usecase
        .list_company(&query)
        .await
        .map_err(map_usecase_company_error)?;

    Ok(ResponseSuccess::Pagination(
        page, 
        per_page, 
        company_list_data.total_data as u64, 
        Some(company_list_data.data),
    ))
}
