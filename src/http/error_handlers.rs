use actix_web::{HttpResponse, http::StatusCode};
use serde_json::{json, Value};
use chrono::Utc;
use crate::http::model::ErrorResponse;


impl ErrorResponse {
    pub fn new(status: StatusCode, message: &str, path: &str, errors: Option<Value>) -> Self {
        ErrorResponse {
            status: status.as_u16(),
            message: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            path: path.to_string(),
            errors,
        }
    }

    pub fn to_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .json(self)
    }
}

pub fn format_validation_errors(errors: validator::ValidationErrors) -> Value {
    let mut error_map = serde_json::Map::new();
    for (field, errors) in errors.field_errors() {
        let messages: Vec<String> = errors.iter()
            .map(|e| {
                format!(
                    "{}: {}",
                    e.code,
                    e.message.clone().unwrap_or_else(|| "Validation error".into())
                )
            })
            .collect();
        error_map.insert(field.to_string(), json!(messages));
    }
    json!(error_map)
}

pub fn handle_400_error(message: &str, path: &str, errors: Option<Value>) -> HttpResponse {
    let error_response = ErrorResponse::new(StatusCode::BAD_REQUEST, message, path, errors);
    error_response.to_response()
}

pub fn handle_401_error(message: &str, path: &str) -> HttpResponse {
    let error_response = ErrorResponse::new(StatusCode::UNAUTHORIZED, message, path, None);
    error_response.to_response()
}

pub fn handle_404_error(message: &str, path: &str) -> HttpResponse {
    let error_response = ErrorResponse::new(StatusCode::NOT_FOUND, message, path, None);
    error_response.to_response()
}

pub fn handle_500_error(message: &str, path: &str) -> HttpResponse {
    let error_response = ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, message, path, None);
    error_response.to_response()
}
