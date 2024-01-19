use axum::{Router, Extension};
use reqwest::Client;
use tower_http::cors::{CorsLayer, Any};
use sqlx::postgres::PgPool;
use std::sync::Arc;
mod health;
mod auth;
mod sessions;
mod user;
mod users;
mod chats;

use utoipa::{OpenApi, Modify, openapi::security::{SecurityScheme, HttpAuthScheme, HttpBuilder}};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;
#[utoipauto(
    paths = "./src/http"
)]
#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Checks app health"),
        (name = "users", description = "Actions with users"),
        (name = "chats", description = "Actions with chats"),
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Bearer token",
                // SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
                SecurityScheme::Http(
                    HttpBuilder::new().scheme(HttpAuthScheme::Bearer).bearer_format("JWT").build()
                )
            )
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub client: Client
}

pub async fn router() -> Router {
    let db_url: String = std::env::var("DATABASE_URL")
        .expect("Can not read DATABASE_URL env variable");

    let pool = PgPool::connect(&db_url).await
        .expect("Can not connect to the database");

    let client = Client::new();

    let state = Arc::new(AppState {
        pool, client
    });

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .nest("/health", health::router())
        .nest("/auth", auth::router())
        .nest("/sessions", sessions::router())
        .nest("/user", user::router())
        .nest("/users", users::router())
        .nest("/chats", chats::router())
        .merge(
            SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .layer(cors)
        .layer(Extension(state))
}