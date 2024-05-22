use crate::state::AppState;
use actix_web::web;

pub async fn list_users(data: &web::Data<AppState>) -> Vec<String> {
    let users = data.users.lock().unwrap();
    users.keys().cloned().collect()
}
