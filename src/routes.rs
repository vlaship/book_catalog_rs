use std::sync::Arc;
use axum::extract::{OriginalUri, Path};
use axum::{Json, Router};
use axum::routing::{get, post};
use sqlx::PgPool;
use crate::auth::controller::AuthController;
use crate::auth::model::{SigninRequest, SignupRequest};
use crate::auth::service::AuthService;
use crate::user::controller::UserController;
use crate::user::repo::UserRepo;
use crate::user::service::UserService;

pub fn routes(pool: PgPool) -> Router {
    let user_repo = Arc::new(UserRepo::new(pool.clone()));
    let user_svc = UserService::new(Arc::clone(&user_repo));
    let auth_svc = AuthService::new(Arc::clone(&user_repo));
    let user_ctrl = Arc::new(UserController::new(user_svc));
    let auth_ctrl = Arc::new(AuthController::new(auth_svc));

    let user = Router::new()
        .route("/:login", get({
            let user_ctrl = Arc::clone(&user_ctrl);
            move |path: Path<String>, OriginalUri(uri): OriginalUri| {
                let user_ctrl = Arc::clone(&user_ctrl);
                async move {
                    user_ctrl.get_user_by_login(uri.to_string().as_str(), path.0).await
                }
            }
        }))
        .route("/", get({
            let user_ctrl = Arc::clone(&user_ctrl);
            move |OriginalUri(uri): OriginalUri| {
                let user_ctrl = Arc::clone(&user_ctrl);
                async move {
                    user_ctrl.get_users(uri.to_string().as_str()).await
                }
            }
        }));

    let auth = Router::new()
        .route("/signup", post({
            let auth_ctrl = Arc::clone(&auth_ctrl);
            move |OriginalUri(uri): OriginalUri, Json(dto): Json<SignupRequest>| {
                let auth_ctrl = Arc::clone(&auth_ctrl);
                async move {
                    auth_ctrl.signup(uri.to_string().as_str(), dto).await
                }
            }
        }))
        .route("/signin", post({
            let auth_ctrl = Arc::clone(&auth_ctrl);
            move |OriginalUri(uri): OriginalUri, Json(dto): Json<SigninRequest>| {
                let auth_ctrl = Arc::clone(&auth_ctrl);
                async move {
                    auth_ctrl.signin(uri.to_string().as_str(), dto).await
                }
            }
        }));

    Router::new()
        .nest("/user", user)
        .nest("/auth", auth)
}
