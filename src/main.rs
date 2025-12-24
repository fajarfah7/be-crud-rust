use axum::{Router, routing::get, routing::post};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

mod company;
mod helper;
mod request;
mod response;

use company::handler::company_handler::{create_company_handler, get_companies_handler};
use company::repository::company_repository_sqlx::CompanyRepositorySqlx;
use company::usecase::company_usecase::CompanyUsecase;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // tracing_subscriber::fmt::init();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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
        .route("/companies", post(create_company_handler))
        .route("/companies", get(get_companies_handler))
        .with_state(usecase);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
