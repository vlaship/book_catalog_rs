use actix_web::{web, HttpRequest};
use sqlx::PgPool;
use web::ServiceConfig;
use crate::auth::controller::AuthController;
use crate::auth::model::{SigninRequest, SignupRequest};
use crate::auth::service::AuthService;
use crate::user::controller::UserController;
use crate::user::repo::UserRepo;
use crate::user::service::UserService;

pub fn configure(cfg: &mut ServiceConfig, pool: &PgPool) {
    let user_repo = UserRepo::new(pool.clone());
    let user_svc = UserService::new(user_repo.clone());
    let user_ctrl = UserController::new(user_svc);
    let auth_svc = AuthService::new(user_repo.clone());
    let auth_ctrl = AuthController::new(auth_svc);

    let user_ctrl_clone = user_ctrl.clone();
    cfg.route("/v1/user/{login}", web::get().to(move |req: HttpRequest, login: web::Path<String>| {
        let user_ctrl = user_ctrl_clone.clone();
        async move {
            user_ctrl.get_user_by_login(req, login).await
        }
    }));

    let user_ctrl_clone = user_ctrl.clone();
    cfg.route("/v1/users", web::get().to(move |req: HttpRequest, | {
        let user_ctrl = user_ctrl_clone.clone();
        async move {
            user_ctrl.get_users(req).await
        }
    }));

    let auth_ctrl_clone = auth_ctrl.clone();
    cfg.route("/v1/auth/signup", web::post().to(move |req: HttpRequest, body: web::Json<SignupRequest>| {
        let auth_ctrl = auth_ctrl_clone.clone();
        async move {
            auth_ctrl.signup(req, body).await
        }
    }));

    let auth_ctrl_clone = auth_ctrl.clone();
    cfg.route("/v1/auth/signin", web::post().to(move |req: HttpRequest, body: web::Json<SigninRequest>| {
        let auth_ctrl = auth_ctrl_clone.clone();
        async move {
            auth_ctrl.signin(req, body).await
        }
    }));
}

pub fn config_factory(pool: PgPool) -> impl FnOnce(&mut ServiceConfig) + Clone {
    move |cfg: &mut ServiceConfig| configure(cfg, &pool)
}
