use crate::user::service::UserService;
use axum::{response::IntoResponse, Json};
use crate::http::error_handlers::handle_error;

pub struct UserController {
    svc: UserService,
}

impl UserController {
    pub fn new(user_service: UserService) -> Self {
        Self { svc: user_service }
    }

    pub async fn get_users(&self, path: &str) -> impl IntoResponse {
        match self.svc.list_users().await {
            Ok(users) => (http::StatusCode::OK, Json(users)).into_response(),
            Err(e) => {
                handle_error(e, path).into_response()
            }
        }
    }

    pub async fn get_user_by_login(&self, path: &str, login: String) -> impl IntoResponse {
        match self.svc.get_user_by_login(login).await {
            Ok(user) => (http::StatusCode::OK, Json(user)).into_response(),
            Err(e) => {
                handle_error(e, path).into_response()
            }
        }
    }
}