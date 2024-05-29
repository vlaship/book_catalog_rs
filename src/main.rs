mod auth;
mod db;
mod http;
mod routes;
mod user;
mod utils;
mod err;

use dotenv::dotenv;
use log::info;
use std::env;
use axum::Router;
use crate::routes::routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Starting the application...");

    let pool = db::init::init_db_pool().await;
    db::migrate::run_migrations(&pool).await;

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("127.0.0.1:{}", port);

    info!("Binding server to address: {}", bind_address);

    let app = Router::new().nest("/v1", routes(pool));

    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
