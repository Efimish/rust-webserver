use axum::{Router, Extension};
use reqwest::Client;
use tower_http::cors::CorsLayer;
use sqlx::postgres::PgPool;
use std::sync::Arc;
mod health;
mod auth;
mod sessions;
mod user;
mod users;
mod chats;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
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
        let db_url = std::env::var("DATABASE_URL")
            .expect("Can not read DATABASE_URL env variable");

        let pool = PgPool::connect(&db_url).await
            .expect("Can not connect to the database");

        let client = Client::new();

        Self {
            pool, client
        }
    }
}