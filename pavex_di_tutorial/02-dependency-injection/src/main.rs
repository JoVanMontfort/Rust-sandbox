use pavex::blueprint::{router, Blueprint};
use pavex::f;
use pavex::lifecycle;
use secrecy::Secret;
use shared_dependencies::{json_response, AuthConfig, DatabaseConfig, User};
use std::sync::Arc;
use uuid::Uuid;

// Dependency types
#[derive(Clone)]
pub struct DatabasePool;

#[derive(Clone)]
pub struct HttpClient;

#[derive(Clone)]
pub struct RedisPool;

// Business services
#[derive(Clone)]
pub struct UserService {
    db: DatabasePool,
    redis: RedisPool,
}

#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    config: AuthConfig,
}

// Request-scoped dependencies
pub struct UserSession {
    pub user_id: String,
    pub permissions: Vec<String>,
}

pub struct RequestLogger {
    pub request_id: String,
}

// Constructors
pub fn app_config() -> DatabaseConfig {
    DatabaseConfig {
        url: Secret::new(
            std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost:5432/mydb".to_string())
        ),
        max_connections: 10,
    }
}

pub fn auth_config() -> AuthConfig {
    AuthConfig {
        jwt_secret: Secret::new(
            std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret".to_string())
        ),
        token_expiry_hours: 24,
    }
}

pub fn database_pool(_config: DatabaseConfig) -> DatabasePool {
    println!("Creating database pool");
    DatabasePool
}

pub fn http_client() -> HttpClient {
    println!("Creating HTTP client");
    HttpClient
}

pub fn redis_pool() -> RedisPool {
    println!("Creating Redis pool");
    RedisPool
}

pub fn user_service(db: DatabasePool, redis: RedisPool) -> UserService {
    UserService { db, redis }
}

pub fn auth_service(user_service: UserService, config: AuthConfig) -> AuthService {
    AuthService { user_service, config }
}

// Request-scoped constructors
pub fn extract_session(request: &pavex::request::RequestHead) -> Result<UserSession, SessionError> {
    let token = request.headers.get("Authorization")
        .ok_or(SessionError::MissingToken)?;

    // In real app, validate JWT token
    Ok(UserSession {
        user_id: "user123".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    })
}

pub fn generate_request_id() -> RequestLogger {
    RequestLogger {
        request_id: Uuid::new_v4().to_string(),
    }
}

// Error handling
#[derive(Debug)]
pub enum SessionError {
    MissingToken,
    InvalidToken,
}

pub fn handle_session_error(_error: &SessionError, _request: &pavex::request::RequestHead) -> pavex::response::Response {
    shared_dependencies::error_response("Authentication required", 401)
}

// Handlers using dependencies
pub async fn get_current_user(
    session: UserSession,
    user_service: UserService,
    logger: RequestLogger,
) -> pavex::response::Response {
    println!("Request ID: {}", logger.request_id);

    let user = User {
        id: session.user_id,
        email: "user@example.com".to_string(),
        name: "Current User".to_string(),
    };

    json_response(user)
}

pub async fn get_user_profile(
    session: UserSession,
    auth_service: AuthService,
    request: pavex::request::RequestHead,
) -> pavex::response::Response {
    let user_id = request.path_params.get("user_id").unwrap();

    let user = User {
        id: user_id.to_string(),
        email: "profile@example.com".to_string(),
        name: "Profile User".to_string(),
    };

    json_response(user)
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();

    // Singleton dependencies
    bp.constructor(f!(crate::app_config), lifecycle::Singleton);
    bp.constructor(f!(crate::auth_config), lifecycle::Singleton);
    bp.constructor(f!(crate::database_pool), lifecycle::Singleton);
    bp.constructor(f!(crate::http_client), lifecycle::Singleton);
    bp.constructor(f!(crate::redis_pool), lifecycle::Singleton);
    bp.constructor(f!(crate::user_service), lifecycle::Singleton);
    bp.constructor(f!(crate::auth_service), lifecycle::Singleton);

    // Request-scoped dependencies
    bp.constructor(f!(crate::extract_session), lifecycle::RequestScoped);
    bp.constructor(f!(crate::generate_request_id), lifecycle::RequestScoped);

    // Error handlers
    bp.error_handler(f!(crate::handle_session_error));

    // Routes
    bp.route(GET, "/me", f!(crate::get_current_user));
    bp.route(GET, "/users/:user_id", f!(crate::get_user_profile));

    bp
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bp = blueprint();
    let server = pavex::hyper::Server::bind(
        "0.0.0.0:8081".parse()?
    ).serve(bp);

    println!("Dependency Injection Example running on http://0.0.0.0:8081");
    println!("Endpoints:");
    println!("  GET /me (requires Authorization header)");
    println!("  GET /users/:user_id (requires Authorization header)");

    server.await?;
    Ok(())
}