use crate::state::AppState;
use crate::user::service::list_users;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = list_users(&data).await;
    HttpResponse::Ok().json(users)
}
