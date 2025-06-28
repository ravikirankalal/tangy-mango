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