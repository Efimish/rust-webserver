//! Definition and initialization of the shared HTTP context.

use std::sync::Arc;
use tokio::sync::Mutex;

use sqlx::PgPool;
use redis::aio::ConnectionManager;
use reqwest::Client;

/// # Shared HTTP context
/// Or "application state".
/// Includes connections to databases and everything that should only be initialized once
/// and shared across the application.
#[derive(Clone)]
pub struct HttpContext {
    /// Postgres pool
    pub pool: PgPool,
    /// Redis pool
    pub redis: Arc<Mutex<ConnectionManager>>,
    /// Reqwest client
    pub client: Client
}

impl HttpContext {
    pub async fn init() -> Self {
        let postgres_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL env variable is not set");

        let redis_url = std::env::var("REDIS_URL")
            .expect("REDIS_URL env variable is not set");

        let pool = PgPool::connect(&postgres_url).await
            .expect("failed to connect to the database");

        let redis_client = redis::Client::open(redis_url)
            .expect("failed to create redis client");

        let redis = ConnectionManager::new(redis_client).await
            .expect("failed to create redis connection");

        let redis = Arc::new(Mutex::new(redis));

        let client = Client::new();

        Self {
            pool, redis, client
        }
    }
}