use async_trait::async_trait;
use shared_dependencies::{RepositoryError, User};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// Define traits
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
    async fn list_all(&self) -> Result<Vec<User>, RepositoryError>;
}

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_welcome_email(&self, user: &User) -> Result<(), RepositoryError>;
}

// In-memory repository for testing/demonstration
pub struct InMemoryUserRepository {
    users: Mutex<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError> {
        let users = self.users.lock().await;
        users.get(id)
            .cloned()
            .ok_or(RepositoryError::NotFound)
    }

    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        let mut users = self.users.lock().await;
        users.insert(user.id.clone(), user.clone());
        println!("[InMemory] Saved user: {} - {}", user.id, user.email);
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        let mut users = self.users.lock().await;
        users.remove(id);
        println!("[InMemory] Deleted user: {}", id);
        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<User>, RepositoryError> {
        let users = self.users.lock().await;
        Ok(users.values().cloned().collect())
    }
}

// Mock email service with rate limiting
pub struct MockEmailService {
    sent_emails: Mutex<Vec<String>>,
    rate_limit: usize,
}

impl MockEmailService {
    pub fn new(rate_limit: usize) -> Self {
        Self {
            sent_emails: Mutex::new(Vec::new()),
            rate_limit,
        }
    }

    pub async fn get_sent_emails(&self) -> Vec<String> {
        self.sent_emails.lock().await.clone()
    }
}

#[async_trait]
impl EmailService for MockEmailService {
    async fn send_welcome_email(&self, user: &User) -> Result<(), RepositoryError> {
        let mut sent_emails = self.sent_emails.lock().await;

        // Check rate limit
        if sent_emails.len() >= self.rate_limit {
            return Err(RepositoryError::Validation("Rate limit exceeded".to_string()));
        }

        sent_emails.push(user.email.clone());
        println!("[MockEmail] Sent welcome email to: {} (total: {})", user.email, sent_emails.len());
        Ok(())
    }
}

// Service with shared state - now implementing Clone
#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<InMemoryUserRepository>,
    email_service: Arc<MockEmailService>,
}

impl UserService {
    pub fn new(user_repo: Arc<InMemoryUserRepository>, email_service: Arc<MockEmailService>) -> Self {
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

    pub async fn list_users(&self) -> Result<Vec<User>, RepositoryError> {
        self.user_repo.list_all().await
    }

    pub async fn get_sent_emails(&self) -> Vec<String> {
        self.email_service.get_sent_emails().await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Arc + Mutex for Shared State ===");

    // Create shared dependencies
    let user_repo = Arc::new(InMemoryUserRepository::new());
    let email_service = Arc::new(MockEmailService::new(3)); // Rate limit: 3 emails

    let user_service = UserService::new(user_repo.clone(), email_service.clone());

    // Register multiple users
    let mut handles = vec![];

    for i in 0..5 {
        let service = user_service.clone(); // This works now!
        let handle = tokio::spawn(async move {
            match service.register_user(&format!("user{}@example.com", i), &format!("User {}", i)).await {
                Ok(user) => println!("✅ Successfully registered: {}", user.email),
                Err(e) => println!("❌ Failed to register user {}: {}", i, e),
            }
        });
        handles.push(handle);
    }

    // Wait for all registrations to complete
    for handle in handles {
        handle.await?;
    }

    // List all users
    println!("\n--- All Registered Users ---");
    let users = user_service.list_users().await?;
    for user in users {
        println!("  - {}: {} ({})", user.id, user.name, user.email);
    }

    // Show sent emails
    println!("\n--- Sent Emails ---");
    let sent_emails = user_service.get_sent_emails().await;
    for email in sent_emails {
        println!("  - {}", email);
    }

    Ok(())
}