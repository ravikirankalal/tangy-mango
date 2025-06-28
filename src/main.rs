mod config;
mod db;
mod models;
mod dao;
mod services;
mod handlers;

use actix_web::{web, App, HttpServer, middleware::Logger};
use std::sync::Arc;

use crate::config::Settings;
use crate::dao::user_dao::UserDao;
use crate::services::user_service::UserService;
use crate::handlers::user_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");
    log::info!("Configuration loaded successfully");

    // Create database connection pool
    let pool = db::create_pool(&settings)
        .await
        .expect("Failed to create database pool");
    log::info!("Database connection pool created");

    // Run migrations
    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");
    log::info!("Database migrations completed");

    // Initialize DAOs and Services
    let user_dao = UserDao::new(pool.clone());
    let user_service = Arc::new(UserService::new(user_dao));

    let server_host = settings.server.host.clone();
    let server_port = settings.server.port;

    log::info!("Starting server at {}:{}", server_host, server_port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(user_service.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/users")
                            .route("", web::post().to(user_handler::create_user))
                            .route("", web::get().to(user_handler::get_users))
                            .route("/{id}", web::get().to(user_handler::get_user))
                    )
            )
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}