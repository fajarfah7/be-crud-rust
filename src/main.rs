use axum::{Router, routing::delete, routing::get, routing::post, routing::put};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

mod company;
mod helper;
mod request;
mod response;

use company::handler::company_handler::{
    create_company_handler, delete_company_handler, get_companies_handler, update_company_handler,
};
use company::repository::company_repository_sqlx::CompanyRepositorySqlx;
use company::usecase::company_usecase::CompanyUsecase;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    // tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::from_default_env())
    //     .init();

    info!("server starting");

    let postgre_address = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgre_address)
        .await
        .unwrap();

    let repo = CompanyRepositorySqlx::new(pool);
    let usecase = Arc::new(CompanyUsecase::new(repo));

    let app = Router::new()
        .route("/company", get(get_companies_handler))
        .route("/company", post(create_company_handler))
        .route("/company/:id", put(update_company_handler))
        .route("/company/:id", delete(delete_company_handler))
        .with_state(usecase);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
