use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub timestamp: String,
    pub path: String,
    pub errors: Option<serde_json::Value>,
}
