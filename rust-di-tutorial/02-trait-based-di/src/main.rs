use async_trait::async_trait;
use shared_dependencies::{RepositoryError, User};
use tokio::time;

// Define traits for our dependencies
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_welcome_email(&self, user: &User) -> Result<(), RepositoryError>;
}

// Concrete implementations
pub struct PostgresUserRepository {
    connection_string: String,
}

impl PostgresUserRepository {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError> {
        println!("[Postgres] Finding user with ID: {}", id);
        // Simulate database call
        time::sleep(time::Duration::from_millis(100)).await;

        Ok(User {
            id: id.to_string(),
            email: "user@example.com".to_string(),
            name: "Postgres User".to_string(),
        })
    }

    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        println!("[Postgres] Saving user: {} - {}", user.id, user.email);
        time::sleep(time::Duration::from_millis(50)).await;
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        println!("[Postgres] Deleting user: {}", id);
        Ok(())
    }
}

pub struct SendgridEmailService {
    api_key: String,
}

impl SendgridEmailService {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl EmailService for SendgridEmailService {
    async fn send_welcome_email(&self, user: &User) -> Result<(), RepositoryError> {
        println!("[Sendgrid] Sending welcome email to: {}", user.email);
        time::sleep(time::Duration::from_millis(200)).await;
        println!("Welcome email sent to {}!", user.email);
        Ok(())
    }
}

// Business service that depends on traits
pub struct UserRegistrationService<R, E>
where
    R: UserRepository,
    E: EmailService,
{
    user_repo: R,
    email_service: E,
}

impl<R, E> UserRegistrationService<R, E>
where
    R: UserRepository,
    E: EmailService,
{
    pub fn new(user_repo: R, email_service: E) -> Self {
        Self {
            user_repo,
            email_service,
        }
    }

    pub async fn register_user(&self, email: &str, name: &str) -> Result<User, RepositoryError> {
        println!("Registering new user: {} - {}", email, name);

        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.to_string(),
            name: name.to_string(),
        };

        // Save user to database
        self.user_repo.save(&user).await?;

        // Send welcome email
        self.email_service.send_welcome_email(&user).await?;

        println!("User registered successfully: {}", user.id);
        Ok(user)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Trait-Based Dependency Injection ===");

    // Create concrete implementations
    let user_repo = PostgresUserRepository::new("postgresql://localhost:5432/mydb".to_string());
    let email_service = SendgridEmailService::new("sg_12345".to_string());

    // Inject dependencies (generic type parameters are inferred)
    let registration_service = UserRegistrationService::new(user_repo, email_service);

    // Use the service
    let user = registration_service.register_user("alice@example.com", "Alice Smith").await?;
    println!("Registered user: {} - {}", user.id, user.name);

    Ok(())
}