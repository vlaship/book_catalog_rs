use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::models::{SignupRequest, SignupResponse, SigninRequest, SigninResponse};
use crate::state::AppState;
use crate::service::{create_user, authenticate_user};
use log::info;
use validator::Validate;
use crate::error_handlers::{handle_400_error, handle_401_error, format_validation_errors};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/v1/auth/signup")
            .route(web::post().to(signup)),
    )
    .service(
        web::resource("/v1/auth/signin")
            .route(web::post().to(signin)),
    );
}

async fn signup(data: web::Data<AppState>, info: web::Json<SignupRequest>, req: HttpRequest) -> impl Responder {
    let signup_request = info.into_inner();

    if let Err(e) = signup_request.validate() {
        return handle_400_error("Validation error", req.path(), Some(format_validation_errors(e)));
    }

    let user_id = create_user(&data, signup_request).await;
    info!("New user signed up with ID: {}", user_id);
    HttpResponse::Ok().json(SignupResponse { id: user_id.to_string() })
}

async fn signin(data: web::Data<AppState>, info: web::Json<SigninRequest>, req: HttpRequest) -> impl Responder {
    let signin_request = info.into_inner();

    if let Err(e) = signin_request.validate() {
        return handle_400_error("Validation error", req.path(), Some(format_validation_errors(e)));
    }

    if let Some(jwt) = authenticate_user(&data, &signin_request).await {
        info!("User signed in: {}", signin_request.login);
        return HttpResponse::Ok().json(SigninResponse { jwt });
    }

    handle_401_error("Invalid login credentials", req.path())
}
