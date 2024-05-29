use crate::user::service::UserService;
use axum::{response::IntoResponse, Json};
use http::Uri;
use crate::http::error_handlers::handle_error;

pub struct UserController {
    svc: UserService,
}

impl UserController {
    pub fn new(user_service: UserService) -> Self {
        Self { svc: user_service }
    }

    pub async fn get_users(&self, uri: Uri) -> impl IntoResponse {
        match self.svc.list_users().await {
            Ok(users) => (http::StatusCode::OK, Json(users)).into_response(),
            Err(e) => {
                let uri = uri.to_string();
                let path = uri.as_str();
                handle_error(e, path).into_response()
            }
        }
    }

    pub async fn get_user_by_login(&self, uri: Uri, login: String) -> impl IntoResponse {
        match self.svc.get_user_by_login(login).await {
            Ok(user) => (http::StatusCode::OK, Json(user)).into_response(),
            Err(e) => {
                let uri = uri.to_string();

                let path = uri.as_str();
                handle_error(e, path).into_response()
            }
        }
    }
}