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