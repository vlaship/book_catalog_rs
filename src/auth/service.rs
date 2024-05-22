use crate::auth::auth_utils::{hash_password, verify_password};
use crate::auth::model::{SigninRequest, SignupRequest};
use crate::utils::id_generator::generate_id;
use crate::state::AppState;
use crate::user::model::User;
use actix_web::web;

pub async fn sign_up(data: &web::Data<AppState>, info: SignupRequest) -> i64 {
    let user_id = generate_id();
    let password_hash = hash_password(&info.password);
    let user = User {
        login: info.login.clone(),
        password_hash,
    };
    let mut users = data.users.lock().unwrap();
    users.insert(info.login.clone(), user);
    user_id
}

pub async fn sign_in(data: &web::Data<AppState>, info: &SigninRequest) -> Option<String> {
    let users = data.users.lock().unwrap();
    if let Some(user) = users.get(&info.login) {
        if verify_password(&info.password, &user.password_hash) {
            return Some("dummy_jwt_token".to_string());
        }
    }
    None
}
