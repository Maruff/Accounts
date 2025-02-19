// Establishes and manages the database connection pool
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5) // Adjust as needed
        .connect(&database_url)
        .await
}

// Function to get the connection pool (for Actix-web state)
pub async fn get_pool() -> PgPool {
    establish_connection().await.expect("Failed to create pool")
}