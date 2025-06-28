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