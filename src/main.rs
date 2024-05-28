mod auth;
mod db;
mod http;
mod routes;
mod user;
mod utils;
mod err;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Starting the application...");

    let pool = db::init::init_db_pool().await;
    db::migrate::run_migrations(&pool).await;

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("127.0.0.1:{}", port);

    info!("Binding server to address: {}", bind_address);

    HttpServer::new(move || {
        App::new().configure(routes::config_factory(pool.clone()))
    })
        .bind(bind_address)?
        .run()
        .await
}
