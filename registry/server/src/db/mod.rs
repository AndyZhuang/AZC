//! Database utilities.

pub use sqlx::PgPool;

/// Create a database connection pool
pub async fn create_pool(database_url: &str) -> sqlx::Result<PgPool> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
}