use async_trait::async_trait;
use shared_dependencies::{RepositoryError, User};
use std::sync::Arc;

// Define traits for our dependencies
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_welcome_email(&self, user: &User) -> Result<(), RepositoryError>;
}

// Concrete implementations
pub struct PostgresUserRepository;
pub struct MySQLUserRepository;
pub struct SendgridEmailService;
pub struct MailgunEmailService;

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError> {
        println!("[Postgres] Finding user: {}", id);
        Ok(User {
            id: id.to_string(),
            email: "postgres@example.com".to_string(),
            name: "Postgres User".to_string(),
        })
    }

    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        println!("[Postgres] Saving user: {}", user.email);
        Ok(())
    }
}

#[async_trait]
impl UserRepository for MySQLUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError> {
        println!("[MySQL] Finding user: {}", id);
        Ok(User {
            id: id.to_string(),
            email: "mysql@example.com".to_string(),
            name: "MySQL User".to_string(),
        })
    }

    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        println!("[MySQL] Saving user: {}", user.email);
        Ok(())
    }
}

#[async_trait]
impl EmailService for SendgridEmailService {
    async fn send_welcome_email(&self, user: &User) -> Result<(), RepositoryError> {
        println!("[Sendgrid] Sending email to: {}", user.email);
        Ok(())
    }
}

#[async_trait]
impl EmailService for MailgunEmailService {
    async fn send_welcome_email(&self, user: &User) -> Result<(), RepositoryError> {
        println!("[Mailgun] Sending email to: {}", user.email);
        Ok(())
    }
}

// Service using dynamic dispatch (trait objects)
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
}

impl UserService {
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
        self.email_service.send_welcome_email(&user).await?;

        Ok(user)
    }

    pub async fn get_user(&self, id: &str) -> Result<User, RepositoryError> {
        self.user_repo.find_by_id(id).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Dynamic Trait Objects ===");

    // Example 1: Postgres + Sendgrid
    println!("\n--- Using Postgres + Sendgrid ---");
    let user_repo = Arc::new(PostgresUserRepository);
    let email_service = Arc::new(SendgridEmailService);
    let user_service = UserService::new(user_repo, email_service);

    let user1 = user_service.register_user("user1@example.com", "User One").await?;
    println!("Registered: {}", user1.email);

    // Example 2: MySQL + Mailgun
    println!("\n--- Using MySQL + Mailgun ---");
    let user_repo = Arc::new(MySQLUserRepository);
    let email_service = Arc::new(MailgunEmailService);
    let user_service = UserService::new(user_repo, email_service);

    let user2 = user_service.register_user("user2@example.com", "User Two").await?;
    println!("Registered: {}", user2.email);

    // Example 3: Runtime decision
    println!("\n--- Runtime Dependency Selection ---");
    let use_postgres = true; // This could come from config
    let use_sendgrid = false;

    let user_repo: Arc<dyn UserRepository> = if use_postgres {
        Arc::new(PostgresUserRepository)
    } else {
        Arc::new(MySQLUserRepository)
    };

    let email_service: Arc<dyn EmailService> = if use_sendgrid {
        Arc::new(SendgridEmailService)
    } else {
        Arc::new(MailgunEmailService)
    };

    let user_service = UserService::new(user_repo, email_service);
    let user3 = user_service.get_user("123").await?;
    println!("Found user: {}", user3.name);

    Ok(())
}