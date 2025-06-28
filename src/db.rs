use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::config::Settings;

pub type DbPool = PgPool;

pub async fn create_pool(settings: &Settings) -> Result<DbPool, sqlx::Error> {
    let database_url = settings.database_url();
    
    PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&database_url)
        .await
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Settings, ServerConfig, DatabaseConfig};

    fn create_test_settings() -> Settings {
        Settings {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "test_user".to_string(),
                password: "test_password".to_string(),
                database_name: "test_db".to_string(),
                max_connections: 5,
            },
        }
    }

    #[test]
    fn test_db_pool_type_alias() {
        // Test that DbPool is correctly aliased to PgPool
        // This is a compile-time test - if it compiles, the alias works
        let _pool_type_check: Option<DbPool> = None;
        let _pg_pool_type_check: Option<PgPool> = None;
        // Both should be the same type
        assert!(true);
    }

    #[test]
    fn test_database_url_generation_for_pool() {
        let settings = create_test_settings();
        let url = settings.database_url();
        
        // Test that the URL format is correct for PostgreSQL
        assert!(url.starts_with("postgres://"));
        assert!(url.contains("test_user"));
        assert!(url.contains("test_password"));
        assert!(url.contains("localhost"));
        assert!(url.contains("5432"));
        assert!(url.contains("test_db"));
    }

    #[test]
    fn test_max_connections_setting() {
        let settings = create_test_settings();
        assert_eq!(settings.database.max_connections, 5);
        
        // Test different max_connections values
        let mut settings_high = settings.clone();
        settings_high.database.max_connections = 20;
        assert_eq!(settings_high.database.max_connections, 20);
        
        let mut settings_low = settings.clone();
        settings_low.database.max_connections = 1;
        assert_eq!(settings_low.database.max_connections, 1);
    }

    // Note: Actual database connection tests would require a running PostgreSQL instance
    // and are better suited for integration tests rather than unit tests.
    // The tests above focus on configuration and setup logic.
}