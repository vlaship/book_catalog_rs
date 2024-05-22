mod routes;
mod state;
mod http;
mod auth;
mod user;
mod utils; 

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use state::AppState;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Starting the application...");

    let app_state = web::Data::new(AppState::new());
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
