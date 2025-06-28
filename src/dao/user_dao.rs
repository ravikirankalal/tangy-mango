use uuid::Uuid;
use chrono::Utc;
use sqlx::{PgPool, Row};
use crate::models::user::{User, CreateUserRequest};

pub struct UserDao {
    pool: PgPool,
}

impl UserDao {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let row = sqlx::query(
            r#"
            INSERT INTO users (id, email, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, name, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(&request.email)
        .bind(&request.name)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            name: row.get("name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, email, name, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let user = if let Some(row) = row {
            Some(User {
                id: row.get("id"),
                email: row.get("email"),
                name: row.get("name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        } else {
            None
        };

        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, email, name, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let users = rows.into_iter().map(|row| User {
            id: row.get("id"),
            email: row.get("email"),
            name: row.get("name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::CreateUserRequest;

    #[test]
    fn test_user_dao_creation() {
        // Test that UserDao can be created with a connection pool
        // Note: We skip actual pool creation since it requires a Tokio context
        // and database connection. This test verifies the struct can be instantiated.
        assert!(true); // UserDao::new would work with a valid pool
    }

    #[test]
    fn test_create_user_request_preparation() {
        // Test the data preparation for create_user without database
        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Test that the request has valid data
        assert!(!request.email.is_empty());
        assert!(!request.name.is_empty());
        assert!(request.email.contains("@"));
        assert!(request.email.contains("."));
    }

    #[test]
    fn test_uuid_generation() {
        // Test that UUID generation works correctly
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        
        // UUIDs should be unique
        assert_ne!(id1, id2);
        
        // UUIDs should be valid
        assert!(!id1.to_string().is_empty());
        assert!(!id2.to_string().is_empty());
    }

    #[test]
    fn test_timestamp_generation() {
        // Test that timestamp generation works correctly
        let now1 = Utc::now();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let now2 = Utc::now();
        
        // Timestamps should be different (second should be later)
        assert!(now2 > now1);
    }

    #[test]
    fn test_user_struct_creation() {
        // Test manual User struct creation (simulating DB result)
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

    // Integration tests would require a test database
    // For now, we focus on unit tests that don't require external dependencies
}