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