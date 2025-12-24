use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{company::handler::types::ProcessCompanyRequest, request::pagination::PaginationRequest};
use crate::company::{
    handler::map_company_error::map_usecase_company_error,
    repository::company_repository::CompanyRepository,
};
use crate::company::{
    handler::map_company_error::validate_company_input, usecase::company_usecase::CompanyUsecase,
};
use crate::response::error::ResponseError;
use crate::response::success::ResponseSuccess;

pub async fn create_company_handler<R: CompanyRepository>(
    State(usecase): State<Arc<CompanyUsecase<R>>>,
    Json(req): Json<ProcessCompanyRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    validate_company_input(&req)?;

    let company = usecase
        .create_company(req.name, req.email, req.code, req.phone_number, req.address)
        .await
        .map_err(map_usecase_company_error)?;

    Ok((StatusCode::CREATED, Json(company)))
}

pub async fn update_company<R: CompanyRepository>(
    State(usecase): State<Arc<CompanyUsecase<R>>>,
    Json(req): Json<ProcessCompanyRequest>,
    Path(id): Path<Uuid>,
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

    Ok((StatusCode::CREATED, Json(company)))
}

pub async fn delete_company<R: CompanyRepository>(
    State(usecase): State<Arc<CompanyUsecase<R>>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ResponseError> {
    usecase
        .delete_company(id)
        .await
        .map_err(map_usecase_company_error)?;

    Ok(StatusCode::OK)
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
    let companies = usecase
        .list_company(&query)
        .await
        .map_err(map_usecase_company_error)?;

    // Ok((StatusCode::OK, Json(companies)))
    Ok(ResponseSuccess::Success(StatusCode::OK, Some(companies)))
}
