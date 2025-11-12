pub use crate::{
    CreateUser, EmailError, EmailService,
    RegistrationError, RepositoryError, UserRegistrationService, UserRepository
};

// Test implementations
#[cfg(test)]
pub mod test_utils {
    use std::sync::Mutex;

    #[derive(Clone, Default)]
    pub struct MockUserRepository {
        pub users: Arc<Mutex<Vec<User>>>,
    }

    impl UserRepository for MockUserRepository {
        fn find_by_id(&self, id: &str) -> Option<User> {
            self.users.lock().unwrap()
                .iter()
                .find(|u| u.id == id)
                .cloned()
        }

        fn create_user(&self, create_user: CreateUser) -> Result<User, RepositoryError> {
            let user = User {
                id: uuid::Uuid::new_v4().to_string(),
                email: create_user.email,
                name: create_user.name,
            };
            self.users.lock().unwrap().push(user.clone());
            Ok(user)
        }
    }

    #[derive(Clone, Default)]
    pub struct MockEmailService {
        pub sent_emails: Arc<Mutex<Vec<User>>>,
    }

    impl EmailService for MockEmailService {
        fn send_welcome_email(&self, user: &User) -> Result<(), EmailError> {
            self.sent_emails.lock().unwrap().push(user.clone());
            Ok(())
        }
    }
}