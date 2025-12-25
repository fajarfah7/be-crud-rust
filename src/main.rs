use axum::{Router, routing::post};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

mod company;
mod app_helper;
mod app_request;
mod app_response;
mod app_middleware;
mod login;

use crate::company::routes::company_routes;
use crate::login::handler::login_handler::login;

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

    let app = Router::new()
        .route("/login", post(login))
        .nest("/company", company_routes(pool));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
