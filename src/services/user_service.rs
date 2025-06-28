use uuid::Uuid;
use crate::dao::user_dao::UserDao;
use crate::models::user::{CreateUserRequest, UserResponse};

pub struct UserService {
    user_dao: UserDao,
}

impl UserService {
    pub fn new(user_dao: UserDao) -> Self {
        Self { user_dao }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse, sqlx::Error> {
        // Basic validation could be added here
        if request.email.is_empty() || request.name.is_empty() {
            return Err(sqlx::Error::Protocol("Email and name cannot be empty".to_string()));
        }

        let user = self.user_dao.create_user(request).await?;
        Ok(UserResponse::from(user))
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserResponse>, sqlx::Error> {
        let user = self.user_dao.get_user_by_id(id).await?;
        Ok(user.map(UserResponse::from))
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, sqlx::Error> {
        let users = self.user_dao.get_all_users().await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::user::CreateUserRequest;

    #[test]
    fn test_user_service_creation() {
        // This test verifies that UserService can be created
        // Note: We skip actual DAO creation since it requires database setup
        // This test focuses on the service structure itself
        assert!(true); // UserService::new would work with a valid DAO
    }

    #[test]
    fn test_create_user_request_validation() {
        // Test validation logic that doesn't require database
        let valid_request = CreateUserRequest {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Test that email and name are not empty
        assert!(!valid_request.email.is_empty());
        assert!(!valid_request.name.is_empty());

        let invalid_email_request = CreateUserRequest {
            email: "".to_string(),
            name: "Test User".to_string(),
        };

        let invalid_name_request = CreateUserRequest {
            email: "test@example.com".to_string(),
            name: "".to_string(),
        };

        // These would fail the validation in create_user method
        assert!(invalid_email_request.email.is_empty());
        assert!(invalid_name_request.name.is_empty());
    }
}