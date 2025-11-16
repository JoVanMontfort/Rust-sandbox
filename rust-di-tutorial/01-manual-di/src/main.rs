use shared_dependencies::User;

// Simple manual dependency injection - passing dependencies explicitly
pub struct DatabaseConnection {
    pub connection_string: String,
}

impl DatabaseConnection {
    pub fn new(connection_string: String) -> Self {
        println!("Creating database connection to: {}", connection_string);
        Self { connection_string }
    }

    pub fn connect(&self) {
        println!("Connected to database: {}", self.connection_string);
    }
}

pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn find_by_id(&self, id: &str) -> Option<User> {
        self.db.connect();
        println!("Finding user with ID: {}", id);

        Some(User {
            id: id.to_string(),
            email: "user@example.com".to_string(),
            name: "John Doe".to_string(),
        })
    }

    pub fn save(&self, user: &User) {
        self.db.connect();
        println!("Saving user: {} - {}", user.id, user.email);
    }
}

pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub fn get_user(&self, id: &str) -> Option<User> {
        self.user_repo.find_by_id(id)
    }

    pub fn create_user(&self, email: &str, name: &str) -> User {
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.to_string(),
            name: name.to_string(),
        };

        self.user_repo.save(&user);
        user
    }
}

fn main() {
    println!("=== Manual Dependency Injection ===");

    // Create dependencies manually
    let db = DatabaseConnection::new("postgresql://localhost:5432/mydb".to_string());
    let user_repo = UserRepository::new(db);
    let user_service = UserService::new(user_repo);

    // Use the service
    let user = user_service.create_user("test@example.com", "Test User");
    println!("Created user: {} - {}", user.id, user.name);

    let found_user = user_service.get_user(&user.id).unwrap();
    println!("Found user: {} - {}", found_user.id, found_user.email);
}