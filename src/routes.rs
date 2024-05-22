use crate::auth::controller::{signin, signup};
use crate::user::controller::get_users;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/v1/auth/signup").route(web::post().to(signup)))
        .service(web::resource("/v1/auth/signin").route(web::post().to(signin)))
        .service(web::resource("/v1/user/list").route(web::get().to(get_users)));
}
