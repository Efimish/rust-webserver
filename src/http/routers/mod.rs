use axum::{Router, Extension};
use reqwest::Client;
use tower_http::cors::CorsLayer;
use sqlx::postgres::PgPool;
use tokio::sync::Mutex;
use redis::aio::ConnectionManager;
use std::sync::Arc;
mod health;
mod auth;
mod sessions;
mod user;
mod users;
mod chats;
/// remove later
mod test;

#[derive(Clone)]
pub struct AppState {
    /// Postgres
    pub pool: PgPool,
    /// Redis
    pub redis: Arc<Mutex<ConnectionManager>>,
    /// Reqwest
    pub client: Client
}

pub async fn router() -> Router {
    let state = Arc::new(AppState::init().await);
    Router::new()
        .nest("/health", health::router())
        .nest("/auth", auth::router())
        .nest("/sessions", sessions::router())
        .nest("/user", user::router())
        .nest("/users", users::router())
        .nest("/chats", chats::router())
        .nest("/test", test::router())
        .layer(cors())
        .layer(Extension(state))
}

fn cors() -> CorsLayer {
    use tower_http::cors::Any;
    CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
}

impl AppState {
    async fn init() -> Self {
        let postgres_url = std::env::var("DATABASE_URL")
            .expect("Can not read DATABASE_URL env variable");

        let redis_url = std::env::var("REDIS_URL")
            .expect("Can not read REDIS_URL env variable");

        let pool = PgPool::connect(&postgres_url).await
            .expect("Can not connect to the database");

        let client = redis::Client::open(redis_url)
            .expect("Can not create redis client");

        let redis = ConnectionManager::new(client).await
            .expect("Can not create redis connection");

        let redis = Arc::new(Mutex::new(redis));

        let client = Client::new();

        Self {
            pool, redis, client
        }
    }
}