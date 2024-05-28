use crate::auth::auth_utils::{hash_password, verify_password};
use crate::auth::model::{SigninRequest, SignupRequest};
use crate::user::repo::{repo_create_user, repo_find_user_by_login};
use sqlx::Error;
use crate::state::AppState;
use crate::user::model::User;
use crate::utils::id_generator::generate_id;
use actix_web::web;

pub async fn sign_up(data: &web::Data<AppState>, info: SignupRequest) -> Result<i64, Error> {
    let user_id = generate_id();
    let password = hash_password(&info.password);
    let new_user = User {
        user_id,
        login: info.login.clone(),
        password,
    };
    repo_create_user(&data.db_pool, &new_user).await?;
    Ok(user_id)
}

pub async fn sign_in(data: &web::Data<AppState>, info: &SigninRequest) -> Option<String> {
    if let Ok(user) = repo_find_user_by_login(&data.db_pool, &info.login).await {
        if verify_password(&info.password, &user.password) {
            return Some("dummy_jwt_token".to_string());
        }
    }
    None
}
