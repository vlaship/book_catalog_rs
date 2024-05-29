use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse},
};
use http::Uri;
use validator::Validate;
use crate::http::error_handlers::{format_validation_errors, handle_400_error, handle_401_error, handle_error};
use crate::auth::model::{SigninRequest, SigninResponse, SignupRequest, SignupResponse};
use crate::auth::service::AuthService;

pub struct AuthController {
    svc: AuthService,
}

impl AuthController {
    pub fn new(auth_service: AuthService) -> Self {
        Self { svc: auth_service }
    }
    pub async fn signup(&self, uri: Uri, dto: SignupRequest) -> impl IntoResponse {
        let uri = uri.to_string();
        let path = uri.as_str();

        if let Err(e) = dto.validate() {
            return handle_400_error("Validation error", path, Some(format_validation_errors(e))).into_response();
        }

        match self.svc.sign_up(dto).await {
            Ok(user_id) => (StatusCode::OK, Json(SignupResponse { id: user_id.to_string() })).into_response(),
            Err(e) => handle_error(e, path).into_response(),
        }
    }

    pub async fn signin(&self, uri: Uri, dto: SigninRequest) -> impl IntoResponse {
        let uri = uri.to_string();
        let path = uri.as_str();

        if let Err(e) = dto.validate() {
            return handle_400_error("Validation error", path, Some(format_validation_errors(e))).into_response();
        }

        match self.svc.sign_in(&dto).await {
            Some(jwt) => (StatusCode::OK, Json(SigninResponse { jwt })).into_response(),
            None => handle_401_error("Unauthorized", path).into_response(),
        }
    }
}