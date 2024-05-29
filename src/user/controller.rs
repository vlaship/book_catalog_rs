use crate::user::service::UserService;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::web::Path;
use crate::http::error_handlers::{handle_404_error, handle_500_error, handle_error};

pub struct UserController {
    svc: UserService,
}

impl UserController {
    pub fn new(user_service: UserService) -> Self {
        Self { svc: user_service }
    }

    pub async fn get_users(&self, req: HttpRequest) -> impl Responder {
        match self.svc.list_users().await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(e) => handle_error(e, req.path()),
        }
    }

    pub async fn get_user_by_login(&self, req: HttpRequest, user_login: Path<String>) -> impl Responder {
        match self.svc.get_user_by_login(&user_login).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(e) => handle_error(e, req.path()),
        }
    }
}
