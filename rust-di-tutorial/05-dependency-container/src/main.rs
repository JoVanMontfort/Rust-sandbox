use async_trait::async_trait;
use shared_dependencies::{RepositoryError, User};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

// Simple dependency container
pub struct Container {
    services: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: 'static + Send + Sync>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

// Our services
#[derive(Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Clone)]
pub struct EmailConfig {
    pub api_key: String,
    pub from_address: String,
}

pub struct ConfigService {
    pub database: DatabaseConfig,
    pub email: EmailConfig,
}

impl ConfigService {
    pub fn new(database: DatabaseConfig, email: EmailConfig) -> Self {
        Self { database, email }
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
}

pub struct SqlUserRepository {
    config: DatabaseConfig,
}

impl SqlUserRepository {
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl UserRepository for SqlUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError> {
        println!("[SQL] Finding user {} with config: {}", id, self.config.url);
        Ok(User {
            id: id.to_string(),
            email: "sql@example.com".to_string(),
            name: "SQL User".to_string(),
        })
    }

    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        println!("[SQL] Saving user {} with config: {}", user.email, self.config.url);
        Ok(())
    }
}

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), RepositoryError>;
}

pub struct SmtpEmailService {
    config: EmailConfig,
}

impl SmtpEmailService {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl EmailService for SmtpEmailService {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), RepositoryError> {
        println!("[SMTP] Sending email to {} from {}: {}", to, self.config.from_address, subject);
        Ok(())
    }
}

// Business service that gets dependencies from container
pub struct UserRegistrationService {
    user_repo: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
}

impl UserRegistrationService {
    pub fn new(user_repo: Arc<dyn UserRepository>, email_service: Arc<dyn EmailService>) -> Self {
        Self {
            user_repo,
            email_service,
        }
    }

    pub async fn register_user(&self, email: &str, name: &str) -> Result<User, RepositoryError> {
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.to_string(),
            name: name.to_string(),
        };

        self.user_repo.save(&user).await?;
        self.email_service.send_email(&user.email, "Welcome!", "Thanks for registering!").await?;

        Ok(user)
    }
}

// Factory function to build the service from container
pub fn create_user_service(container: &Container) -> Result<UserRegistrationService, &'static str> {
    let config = container.get::<ConfigService>()
        .ok_or("ConfigService not found in container")?;

    let user_repo = Arc::new(SqlUserRepository::new(config.database.clone()));
    let email_service = Arc::new(SmtpEmailService::new(config.email.clone()));

    Ok(UserRegistrationService::new(user_repo, email_service))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Dependency Container ===");

    // Setup container with dependencies
    let mut container = Container::new();

    let config = ConfigService::new(
        DatabaseConfig {
            url: "postgresql://localhost:5432/app".to_string(),
            max_connections: 10,
        },
        EmailConfig {
            api_key: "smtp_12345".to_string(),
            from_address: "noreply@example.com".to_string(),
        },
    );

    container.register(config);

    // Create service using container
    let user_service = create_user_service(&container)?;

    // Use the service
    let user = user_service.register_user("container@example.com", "Container User").await?;
    println!("Registered user: {} - {}", user.id, user.name);

    // Demonstrate container usage
    if let Some(config) = container.get::<ConfigService>() {
        println!("Database URL from container: {}", config.database.url);
        println!("Email from address: {}", config.email.from_address);
    }

    Ok(())
}