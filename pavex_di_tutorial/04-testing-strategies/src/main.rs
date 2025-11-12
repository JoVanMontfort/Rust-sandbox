use pavex::blueprint::{router, Blueprint};
use pavex::f;
use pavex::lifecycle;
use shared_dependencies::{json_response, User};
use std::sync::Arc;

// Traits for testability
pub trait UserRepository: Send + Sync {
    fn find_by_id(&self, id: &str) -> Option<User>;
    fn create_user(&self, user: CreateUser) -> Result<User, RepositoryError>;
}

pub trait EmailService: Send + Sync {
    fn send_welcome_email(&self, user: &User) -> Result<(), EmailError>;
}

// Real implementations
#[derive(Clone)]
pub struct PostgresUserRepository;

impl UserRepository for PostgresUserRepository {
    fn find_by_id(&self, id: &str) -> Option<User> {
        // Real database call
        Some(User {
            id: id.to_string(),
            email: "real@example.com".to_string(),
            name: "Real User".to_string(),
        })
    }

    fn create_user(&self, _user: CreateUser) -> Result<User, RepositoryError> {
        // Real database call
        Ok(User {
            id: "123".to_string(),
            email: "new@example.com".to_string(),
            name: "New User".to_string(),
        })
    }
}

#[derive(Clone)]
pub struct SendgridEmailService;

impl EmailService for SendgridEmailService {
    fn send_welcome_email(&self, _user: &User) -> Result<(), EmailError> {
        // Real email sending
        println!("Sending welcome email via Sendgrid");
        Ok(())
    }
}

// Business service
pub struct UserRegistrationService {
    user_repo: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
}

#[derive(Debug)]
pub struct RepositoryError;
#[derive(Debug)]
pub struct EmailError;

impl UserRegistrationService {
    pub fn register_user(&self, create_user: CreateUser) -> Result<User, RegistrationError> {
        let user = self.user_repo.create_user(create_user)
            .map_err(|_| RegistrationError::Repository)?;

        self.email_service.send_welcome_email(&user)
            .map_err(|_| RegistrationError::Email)?;

        Ok(user)
    }
}

#[derive(Debug)]
pub enum RegistrationError {
    Repository,
    Email,
}

// Constructors
pub fn user_repository() -> Arc<dyn UserRepository> {
    Arc::new(PostgresUserRepository)
}

pub fn email_service() -> Arc<dyn EmailService> {
    Arc::new(SendgridEmailService)
}

pub fn user_registration_service(
    user_repo: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
) -> UserRegistrationService {
    UserRegistrationService {
        user_repo,
        email_service,
    }
}

// Handlers
pub async fn register_user_handler(
    request: pavex::request::RequestHead,
    service: UserRegistrationService,
) -> Result<pavex::response::Response, RegistrationError> {
    let body = request.body.bytes().await?;
    let create_user: CreateUser = serde_json::from_slice(&body)
        .map_err(|_| RegistrationError::Repository)?;

    let user = service.register_user(create_user)?;
    Ok(json_response(user))
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();

    bp.constructor(f!(crate::user_repository), lifecycle::Singleton);
    bp.constructor(f!(crate::email_service), lifecycle::Singleton);
    bp.constructor(f!(crate::user_registration_service), lifecycle::Singleton);

    bp.route(POST, "/register", f!(crate::register_user_handler));

    bp
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bp = blueprint();
    let server = pavex::hyper::Server::bind(
        "0.0.0.0:8083".parse()?
    ).serve(bp);

    println!("Testing Strategies Example running on http://0.0.0.0:8083");
    println!("Endpoint: POST /register");

    server.await?;
    Ok(())
}