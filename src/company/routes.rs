use std::sync::Arc;

use crate::app_middleware::atuh_middleware::auth_middleware;
use crate::company::handler::company_handler::{
    create_company_handler, delete_company_handler, get_companies_handler, update_company_handler,
};
use crate::company::repository::company_repository_sqlx::CompanyRepositorySqlx;
use crate::company::usecase::company_usecase::CompanyUsecase;
use axum::{Router, routing::delete, routing::get, routing::post, routing::put};
use axum::middleware;
use sqlx::{Pool, Postgres};

pub fn company_routes(pool: Pool<Postgres>) -> Router {
    let repo = CompanyRepositorySqlx::new(pool);
    let usecase = Arc::new(CompanyUsecase::new(repo));

    Router::new()
        .route("/", get(get_companies_handler))
        .route("/", post(create_company_handler))
        .route("/:id", put(update_company_handler))
        .route("/:id", delete(delete_company_handler))
        .with_state(usecase)
        .layer(middleware::from_fn(auth_middleware))
}
