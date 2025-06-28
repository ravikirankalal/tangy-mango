use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_create_user_request_creation() {
        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        assert_eq!(request.email, "test@example.com");
        assert_eq!(request.name, "Test User");
    }

    #[test]
    fn test_user_creation() {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let user = User {
            id,
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(user.id, id);
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.created_at, now);
        assert_eq!(user.updated_at, now);
    }

    #[test]
    fn test_user_to_user_response_conversion() {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let user = User {
            id,
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            created_at: now,
            updated_at: now,
        };

        let response: UserResponse = user.clone().into();

        assert_eq!(response.id, user.id);
        assert_eq!(response.email, user.email);
        assert_eq!(response.name, user.name);
        assert_eq!(response.created_at, user.created_at);
        assert_eq!(response.updated_at, user.updated_at);
    }

    #[test]
    fn test_user_response_creation() {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let response = UserResponse {
            id,
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(response.id, id);
        assert_eq!(response.email, "test@example.com");
        assert_eq!(response.name, "Test User");
        assert_eq!(response.created_at, now);
        assert_eq!(response.updated_at, now);
    }
}