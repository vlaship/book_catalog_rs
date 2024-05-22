use crate::state::AppState;
use crate::models::{SignupRequest, SigninRequest};
use snowflake::SnowflakeIdGenerator;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::env;
use actix_web::web;

lazy_static! {
    static ref ID_GENERATOR: Mutex<SnowflakeIdGenerator> = {
        let node_id: i32 = env::var("NODE_ID").unwrap_or_else(|_| "1".to_string()).parse().expect("NODE_ID must be a number");
        let machine_id: i32 = env::var("MACHINE_ID").unwrap_or_else(|_| "1".to_string()).parse().expect("MACHINE_ID must be a number");
        Mutex::new(SnowflakeIdGenerator::new(node_id, machine_id))
    };
}

pub async fn create_user(data: &web::Data<AppState>, info: SignupRequest) -> i64 {
    let user_id = ID_GENERATOR.lock().unwrap().real_time_generate();
    let mut users = data.users.lock().unwrap();
    users.insert(info.login, info.password);
    user_id
}

pub async fn authenticate_user(data: &web::Data<AppState>, info: &SigninRequest) -> Option<String> {
    let users = data.users.lock().unwrap();
    if let Some(stored_password) = users.get(&info.login) {
        if stored_password == &info.password {
            return Some("dummy_jwt_token".to_string());
        }
    }
    None
}
