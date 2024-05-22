use std::collections::HashMap;
use std::sync::Mutex;
use crate::user::model::User;

pub struct AppState {
    pub users: Mutex<HashMap<String, User>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            users: Mutex::new(HashMap::new()),
        }
    }
}
