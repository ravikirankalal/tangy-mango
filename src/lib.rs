// Library module to expose functionality for integration tests

pub mod config;
pub mod db;
pub mod models;
pub mod dao;
pub mod services;
pub mod handlers;

// Re-export commonly used types for easier testing
pub use config::Settings;
pub use models::user::{User, CreateUserRequest, UserResponse};
pub use dao::user_dao::UserDao;
pub use services::user_service::UserService;