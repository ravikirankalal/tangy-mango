use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("Config"))
            .build()?;

        s.try_deserialize()
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                max_connections: 10,
            },
        }
    }

    #[test]
    fn test_server_config_creation() {
        let server_config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };

        assert_eq!(server_config.host, "127.0.0.1");
        assert_eq!(server_config.port, 8080);
    }

    #[test]
    fn test_database_config_creation() {
        let db_config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "test_user".to_string(),
            password: "test_password".to_string(),
            database_name: "test_db".to_string(),
            max_connections: 10,
        };

        assert_eq!(db_config.host, "localhost");
        assert_eq!(db_config.port, 5432);
        assert_eq!(db_config.username, "test_user");
        assert_eq!(db_config.password, "test_password");
        assert_eq!(db_config.database_name, "test_db");
        assert_eq!(db_config.max_connections, 10);
    }

    #[test]
    fn test_database_url_generation() {
        let settings = create_test_settings();
        let database_url = settings.database_url();
        
        let expected_url = "postgres://test_user:test_password@localhost:5432/test_db";
        assert_eq!(database_url, expected_url);
    }

    #[test]
    fn test_database_url_with_special_characters() {
        let mut settings = create_test_settings();
        settings.database.password = "p@ssw0rd!".to_string();
        
        let database_url = settings.database_url();
        assert!(database_url.contains("p@ssw0rd!"));
    }

    #[test]
    fn test_settings_clone() {
        let settings = create_test_settings();
        let cloned_settings = settings.clone();
        
        assert_eq!(settings.server.host, cloned_settings.server.host);
        assert_eq!(settings.server.port, cloned_settings.server.port);
        assert_eq!(settings.database.host, cloned_settings.database.host);
        assert_eq!(settings.database.port, cloned_settings.database.port);
    }

    #[test]
    fn test_different_port_configurations() {
        let mut settings = create_test_settings();
        
        // Test different server ports
        settings.server.port = 3000;
        assert_eq!(settings.server.port, 3000);
        
        // Test different database ports
        settings.database.port = 5433;
        assert_eq!(settings.database.port, 5433);
        
        let url = settings.database_url();
        assert!(url.contains(":5433/"));
    }
}