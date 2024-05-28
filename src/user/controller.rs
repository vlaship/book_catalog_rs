use crate::state::AppState;
use crate::user::service::svc_list_users;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    match svc_list_users(&data).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
