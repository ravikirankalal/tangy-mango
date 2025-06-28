use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::models::user::CreateUserRequest;
use crate::services::user_service::UserService;

// Error response structure
#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn create_user(
    user_service: web::Data<UserService>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    match user_service.create_user(request.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(err) => {
            log::error!("Failed to create user: {}", err);
            Ok(HttpResponse::BadRequest().json(ErrorResponse {
                error: "Failed to create user".to_string(),
            }))
        }
    }
}

pub async fn get_user(
    user_service: web::Data<UserService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            error: "User not found".to_string(),
        })),
        Err(err) => {
            log::error!("Failed to get user: {}", err);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            }))
        }
    }
}

pub async fn get_users(
    user_service: web::Data<UserService>,
) -> Result<HttpResponse> {
    match user_service.get_all_users().await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(err) => {
            log::error!("Failed to get users: {}", err);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_error_response_creation() {
        let error_response = ErrorResponse {
            error: "Test error message".to_string(),
        };

        assert_eq!(error_response.error, "Test error message");
    }

    #[test]
    fn test_error_response_serialization() {
        let error_response = ErrorResponse {
            error: "Test error message".to_string(),
        };

        let json = serde_json::to_string(&error_response).unwrap();
        assert!(json.contains("Test error message"));
        assert!(json.contains("error"));
    }

    #[test]
    fn test_uuid_parsing() {
        // Test UUID parsing that would happen in path extraction
        let valid_uuid_str = "123e4567-e89b-12d3-a456-426614174000";
        let uuid_result = Uuid::parse_str(valid_uuid_str);
        assert!(uuid_result.is_ok());

        let invalid_uuid_str = "not-a-uuid";
        let invalid_result = Uuid::parse_str(invalid_uuid_str);
        assert!(invalid_result.is_err());
    }

    #[test]
    fn test_create_user_request_deserialization() {
        let json_data = r#"{"email": "test@example.com", "name": "Test User"}"#;
        let request: Result<CreateUserRequest, _> = serde_json::from_str(json_data);
        
        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.name, "Test User");
    }

    #[test]
    fn test_create_user_request_invalid_json() {
        let invalid_json = r#"{"email": "test@example.com"}"#; // missing name field
        let request: Result<CreateUserRequest, _> = serde_json::from_str(invalid_json);
        
        assert!(request.is_err()); // Should fail due to missing required field
    }

    // Note: Full integration tests for handlers would require setting up
    // a test application with mock services, which is more complex.
    // The tests above focus on the core logic and data handling.
}