//! AZC Package Registry Server
//!
//! Central repository for publishing, discovering, and downloading AZC packages.

mod auth;
mod db;
mod handlers;
mod models;
mod storage;

use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "azc_registry=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // Storage client
    let storage = storage::Storage::new().await?;

    // Build app
    let app = Router::new()
        // Public routes
        .route("/api/v1/packages", get(handlers::list_packages))
        .route("/api/v1/packages/:name", get(handlers::get_package))
        .route("/api/v1/packages/:name/versions", get(handlers::list_versions))
        .route("/api/v1/packages/:name/:version", get(handlers::get_version))
        .route("/api/v1/search", get(handlers::search_packages))
        .route("/api/v1/users/:username", get(handlers::get_user))
        
        // Protected routes (require auth)
        .route("/api/v1/packages/new", put(handlers::publish_package))
        .route("/api/v1/users/register", post(handlers::register_user))
        
        // Health check
        .route("/health", get(|| async { "OK" }))
        
        // Layers
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        
        // State
        .with_state(handlers::AppState { pool, storage });

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("AZC Registry listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}