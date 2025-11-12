use pavex::blueprint::{router, Blueprint};
use pavex::f;
use pavex::lifecycle;
use shared_dependencies::{error_response, json_response};
use std::time::Instant;

// Custom error types
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Authentication required")]
    Unauthorized,
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

// Request context
pub struct RequestContext {
    pub start_time: Instant,
    pub request_id: String,
    pub user_agent: Option<String>,
}

// Response logger
pub struct ResponseLogger {
    pub status_code: u16,
    pub duration: u128,
}

// Pre-processors (middleware)
pub fn extract_user_agent(request: &pavex::request::RequestHead) -> Option<String> {
    request.headers.get("User-Agent")
        .map(|ua| ua.to_string())
}

pub fn create_request_context(
    request: &pavex::request::RequestHead,
) -> RequestContext {
    RequestContext {
        start_time: Instant::now(),
        request_id: uuid::Uuid::new_v4().to_string(),
        user_agent: extract_user_agent(request),
    }
}

// Post-processors (middleware)
pub fn log_response(
    response: &mut pavex::response::Response,
    context: &RequestContext,
) -> ResponseLogger {
    let duration = context.start_time.elapsed().as_millis();
    let status_code = response.status().as_u16();

    println!(
        "Request {} - {} - {}ms - User-Agent: {:?}",
        context.request_id,
        status_code,
        duration,
        context.user_agent
    );

    ResponseLogger {
        status_code,
        duration,
    }
}

// Error handlers
pub fn handle_unauthorized(
    _error: &ApiError,
    _request: &pavex::request::RequestHead,
) -> pavex::response::Response {
    error_response("Authentication required", 401)
}

pub fn handle_not_found(
    error: &ApiError,
    _request: &pavex::request::RequestHead,
) -> pavex::response::Response {
    if let ApiError::NotFound(resource) = error {
        error_response(&format!("Resource not found: {}", resource), 404)
    } else {
        error_response("Not found", 404)
    }
}

pub fn handle_validation_error(
    error: &ApiError,
    _request: &pavex::request::RequestHead,
) -> pavex::response::Response {
    if let ApiError::Validation(message) = error {
        error_response(&format!("Validation error: {}", message), 400)
    } else {
        error_response("Bad request", 400)
    }
}

pub fn handle_internal_error(
    _error: &ApiError,
    _request: &pavex::request::RequestHead,
) -> pavex::response::Response {
    error_response("Internal server error", 500)
}

// Example handlers that might fail
pub async fn get_user(
    request: pavex::request::RequestHead,
    context: RequestContext,
) -> Result<pavex::response::Response, ApiError> {
    let user_id = request.path_params.get("user_id")
        .ok_or_else(|| ApiError::Validation("Missing user_id".to_string()))?;

    if user_id == "admin" {
        return Err(ApiError::NotFound(format!("User {}", user_id)));
    }

    if user_id == "error" {
        return Err(ApiError::Internal);
    }

    let user = shared_dependencies::User {
        id: user_id.to_string(),
        email: "user@example.com".to_string(),
        name: "Example User".to_string(),
    };

    Ok(json_response(user))
}

pub async fn create_user(
    _request: pavex::request::RequestHead,
    _context: RequestContext,
) -> Result<pavex::response::Response, ApiError> {
    // Simulate validation error
    Err(ApiError::Validation("Email is required".to_string()))
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();

    // Pre-processors (run before handlers)
    bp.pre_process(f!(crate::create_request_context));

    // Post-processors (run after handlers)
    bp.post_process(f!(crate::log_response));

    // Error handlers
    bp.error_handler(f!(crate::handle_unauthorized));
    bp.error_handler(f!(crate::handle_not_found));
    bp.error_handler(f!(crate::handle_validation_error));
    bp.error_handler(f!(crate::handle_internal_error));

    // Routes
    bp.route(GET, "/users/:user_id", f!(crate::get_user));
    bp.route(POST, "/users", f!(crate::create_user));

    bp
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bp = blueprint();
    let server = pavex::hyper::Server::bind(
        "0.0.0.0:8082".parse()?
    ).serve(bp);

    println!("Middleware and Errors Example running on http://0.0.0.0:8082");
    println!("Endpoints:");
    println!("  GET /users/:user_id - try 'admin', 'error', or any other ID");
    println!("  POST /users - always returns validation error");
    println!();
    println!("Observe the request logging in the console!");

    server.await?;
    Ok(())
}