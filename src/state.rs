use std::sync::Mutex;
use std::collections::HashMap;

pub struct AppState {
    pub users: Mutex<HashMap<String, String>>, // Simple in-memory store for example purposes
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            users: Mutex::new(HashMap::new()),
        }
    }
}
