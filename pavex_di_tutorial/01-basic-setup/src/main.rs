use pavex::blueprint::{router, Blueprint};
use pavex::f;
use shared_dependencies::{json_response, HealthResponse};

// Basic handler without dependencies
pub async fn health_check() -> pavex::response::Response {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    json_response(response)
}

// Handler with path parameters
pub async fn get_user(
    request: pavex::request::RequestHead,
) -> pavex::response::Response {
    let user_id = request.path_params.get("user_id").unwrap();
    let user = shared_dependencies::User {
        id: user_id.to_string(),
        email: "user@example.com".to_string(),
        name: "John Doe".to_string(),
    };
    json_response(user)
}

// Handler with query parameters
pub async fn search_users(
    request: pavex::request::RequestHead,
) -> pavex::response::Response {
    let query = request.query_params.get("q").unwrap_or("");
    let users = vec![
        shared_dependencies::User {
            id: "1".to_string(),
            email: "user1@example.com".to_string(),
            name: "User One".to_string(),
        }
    ];
    json_response(users)
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();

    // Register routes
    bp.route(GET, "/health", f!(crate::health_check));
    bp.route(GET, "/users/:user_id", f!(crate::get_user));
    bp.route(GET, "/users", f!(crate::search_users));

    bp
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bp = blueprint();
    let server = pavex::hyper::Server::bind(
        "0.0.0.0:8080".parse()?
    ).serve(bp);

    println!("Server running on http://0.0.0.0:8080");
    println!("Endpoints:");
    println!("  GET /health");
    println!("  GET /users/:user_id");
    println!("  GET /users?q=search");

    server.await?;
    Ok(())
}