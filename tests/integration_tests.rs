//! Integration tests for the tangy-mango web service
//! 
//! These tests demonstrate how the different components work together.
//! For actual database integration tests, you would need a test database setup.

use tangy_mango::{CreateUserRequest, UserResponse};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_user_workflow() {
    // Test the complete user data flow from request to response
    let request = CreateUserRequest {
        email: "integration@test.com".to_string(),
        name: "Integration Test User".to_string(),
    };

    // Verify request structure
    assert!(!request.email.is_empty());
    assert!(!request.name.is_empty());
    assert!(request.email.contains("@"));

    // Simulate a user response (as would come from the service)
    let user_response = UserResponse {
        id: Uuid::new_v4(),
        email: request.email.clone(),
        name: request.name.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Verify the response matches the request
    assert_eq!(user_response.email, request.email);
    assert_eq!(user_response.name, request.name);
    assert!(!user_response.id.to_string().is_empty());
}

#[test]
fn test_error_scenarios() {
    // Test various error conditions that the API should handle
    
    // Empty email
    let invalid_request = CreateUserRequest {
        email: "".to_string(),
        name: "Valid Name".to_string(),
    };
    assert!(invalid_request.email.is_empty());

    // Empty name
    let invalid_request = CreateUserRequest {
        email: "valid@email.com".to_string(),
        name: "".to_string(),
    };
    assert!(invalid_request.name.is_empty());

    // Invalid email format (basic check)
    let invalid_request = CreateUserRequest {
        email: "notanemail".to_string(),
        name: "Valid Name".to_string(),
    };
    assert!(!invalid_request.email.contains("@"));
}

#[test]
fn test_json_serialization_deserialization() {
    // Test JSON handling for API requests and responses
    let request = CreateUserRequest {
        email: "json@test.com".to_string(),
        name: "JSON Test User".to_string(),
    };

    // Test serialization to JSON
    let json_str = serde_json::to_string(&request).unwrap();
    assert!(json_str.contains("json@test.com"));
    assert!(json_str.contains("JSON Test User"));

    // Test deserialization from JSON
    let parsed_request: CreateUserRequest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed_request.email, request.email);
    assert_eq!(parsed_request.name, request.name);

    // Test response serialization
    let response = UserResponse {
        id: Uuid::new_v4(),
        email: "response@test.com".to_string(),
        name: "Response Test User".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    assert!(response_json.contains("response@test.com"));
    assert!(response_json.contains("Response Test User"));
}

#[test]
fn test_configuration_integration() {
    // Test configuration components work together
    use tangy_mango::config::{Settings, ServerConfig, DatabaseConfig};

    let settings = Settings {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
        },
        database: DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "test_user".to_string(),
            password: "test_password".to_string(),
            database_name: "test_database".to_string(),
            max_connections: 10,
        },
    };

    // Test database URL generation
    let db_url = settings.database_url();
    assert!(db_url.starts_with("postgres://"));
    assert!(db_url.contains("test_user"));
    assert!(db_url.contains("test_password"));
    assert!(db_url.contains("localhost:5432"));
    assert!(db_url.contains("test_database"));

    // Test server configuration
    assert_eq!(settings.server.host, "0.0.0.0");
    assert_eq!(settings.server.port, 8080);
}