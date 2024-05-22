use crate::auth::service::{sign_in, sign_up};
use crate::http::error_handlers::{format_validation_errors, handle_400_error, handle_401_error};
use crate::auth::model::{SigninRequest, SigninResponse, SignupRequest, SignupResponse};
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::info;
use validator::Validate;

pub async fn signup(data: web::Data<AppState>, info: web::Json<SignupRequest>, req: HttpRequest) -> impl Responder {
    let signup_request = info.into_inner();

    if let Err(e) = signup_request.validate() {
        return handle_400_error("Validation error", req.path(), Some(format_validation_errors(e)));
    }

    match sign_up(&data, signup_request).await {
        Ok(user_id) => {
            info!("New user signed up with ID: {}", user_id);
            HttpResponse::Ok().json(SignupResponse { id: user_id.to_string() })
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn signin(data: web::Data<AppState>, info: web::Json<SigninRequest>, req: HttpRequest) -> impl Responder {
    let signin_request = info.into_inner();

    if let Err(e) = signin_request.validate() {
        return handle_400_error("Validation error", req.path(), Some(format_validation_errors(e)));
    }

    if let Some(jwt) = sign_in(&data, &signin_request).await {
        info!("User signed in: {}", signin_request.login);
        return HttpResponse::Ok().json(SigninResponse { jwt });
    }

    handle_401_error("Invalid login credentials", req.path())
}

