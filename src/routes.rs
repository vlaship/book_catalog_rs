use std::sync::Arc;
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
    let user_repo = Arc::new(UserRepo::new(pool.clone()));
    let user_svc = UserService::new(Arc::clone(&user_repo));
    let user_ctrl = Arc::new(UserController::new(user_svc));
    let auth_svc = AuthService::new(Arc::clone(&user_repo));
    let auth_ctrl = Arc::new(AuthController::new(auth_svc));

    cfg.service(
        web::scope("/v1/user")
            .route("/{login}", web::get().to({
                let user_ctrl_cloned = Arc::clone(&user_ctrl);
                move |req: HttpRequest, login: web::Path<String>| {
                    let user_ctrl_cloned = Arc::clone(&user_ctrl_cloned);
                    async move {
                        user_ctrl_cloned.get_user_by_login(req, login).await
                    }
                }
            }))
            .route("/", web::get().to({
                let user_ctrl_cloned = Arc::clone(&user_ctrl);
                move |req: HttpRequest| {
                    let user_ctrl_cloned = Arc::clone(&user_ctrl_cloned);
                    async move {
                        user_ctrl_cloned.get_users(req).await
                    }
                }
            })),
    );

    cfg.service(
        web::scope("/v1/auth")
            .route("/signup", web::post().to({
                let auth_ctrl_cloned = Arc::clone(&auth_ctrl);
                move |req: HttpRequest, body: web::Json<SignupRequest>| {
                    let auth_ctrl_cloned = Arc::clone(&auth_ctrl_cloned);
                    async move {
                        auth_ctrl_cloned.signup(req, body).await
                    }
                }
            }))
            .route("/signin", web::post().to({
                let auth_ctrl_cloned = Arc::clone(&auth_ctrl);
                move |req: HttpRequest, body: web::Json<SigninRequest>| {
                    let auth_ctrl_cloned = Arc::clone(&auth_ctrl_cloned);
                    async move {
                        auth_ctrl_cloned.signin(req, body).await
                    }
                }
            }))
    );
}

pub fn config_factory(pool: PgPool) -> impl FnOnce(&mut ServiceConfig) + Clone {
    move |cfg: &mut ServiceConfig| configure(cfg, &pool)
}
