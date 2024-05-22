mod routes;
mod state;
mod http;
mod auth;
mod user;
mod utils; 
mod db;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use state::AppState;
use log::info;
use sqlx::postgres::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Starting the application...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");

    let app_state = web::Data::new(AppState::new(pool));
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("127.0.0.1:{}", port);

    info!("Binding server to address: {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}

