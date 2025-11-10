use pavex::response::Response;
use secrecy::Secret;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: Secret<String>,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: Secret<String>,
    pub token_expiry_hours: u64,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
}

// Common response helpers
pub fn json_response<T: Serialize>(data: T) -> Response {
    let body = serde_json::to_vec(&data).unwrap();
    Response::ok()
        .set_typed_header("Content-Type: application/json")
        .set_body(body)
}

pub fn error_response(message: &str, code: u16) -> Response {
    let error = ApiError {
        message: message.to_string(),
        code,
    };
    let body = serde_json::to_vec(&error).unwrap();
    Response::new(code)
        .set_typed_header("Content-Type: application/json")
        .set_body(body)
}