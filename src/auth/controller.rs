use crate::auth::service::AuthService;
use crate::http::error_handlers::{format_validation_errors, handle_400_error, handle_401_error, handle_500_error, handle_error};
use crate::auth::model::{SigninRequest, SigninResponse, SignupRequest, SignupResponse};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::info;
use validator::Validate;

#[derive(Clone)]
pub struct AuthController {
    svc: AuthService,
}

impl AuthController {
    pub fn new(auth_service: AuthService) -> Self {
        Self { svc: auth_service }
    }

    pub async fn signup(
        &self,
        req: HttpRequest,
        body: web::Json<SignupRequest>,
    ) -> impl Responder {
        let signup_request = body.into_inner();

        if let Err(e) = signup_request.validate() {
            return handle_400_error("Validation error", req.path(), Some(format_validation_errors(e)));
        }

        match self.svc.sign_up(signup_request).await {
            Ok(user_id) => {
                info!("New user signed up with ID: {}", user_id);
                HttpResponse::Ok().json(SignupResponse { id: user_id.to_string() })
            }
            Err(e) => handle_error(e, req.path()),
        }
    }

    pub async fn signin(
        &self,
        req: HttpRequest,
        body: web::Json<SigninRequest>,
    ) -> impl Responder {
        let signin_request = body.into_inner();

        if let Err(e) = signin_request.validate() {
            return handle_400_error("Validation error", req.path(), Some(format_validation_errors(e)));
        }

        if let Some(jwt) = self.svc.sign_in(&signin_request).await {
            info!("User signed in: {}", signin_request.login);
            return HttpResponse::Ok().json(SigninResponse { jwt });
        }

        handle_401_error("Invalid login credentials", req.path())
    }
}