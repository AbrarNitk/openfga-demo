use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use std::time::Duration;

/// Application context that holds shared resources
#[derive(Clone)]
pub struct Ctx {
    /// PostgreSQL connection pool
    pub db: PgPool,
    /// Application profile name (e.g., "dev", "prod")
    pub profile: String,
}

impl Ctx {
    /// Create a new application context
    pub async fn new() -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        // Load environment variables from .env file if it exists
        dotenv::dotenv().ok();

        // Get profile name from environment, default to "dev"
        let profile = env::var("PROFILE").unwrap_or_else(|_| "dev".to_string());
        tracing::info!("Starting application with profile: {}", profile);

        // Create database connection pool
        let db = pg_pool().await?;

        Ok(Arc::new(Self { db, profile }))
    }
}

async fn pg_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
    // Get database URL from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::info!("Connecting to database");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    // Test database connection
    sqlx::query("SELECT 1").execute(&db).await?;
    tracing::info!("Database connection established successfully");

    Ok(db)
}
