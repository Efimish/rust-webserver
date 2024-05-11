//! Routers for the HTTP server.
//! Each router contains routes for a specific part of the API.
//! The main router combines them all into big one.

use axum::{Router, Extension};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::http::HttpContext;
mod fallback;
mod health;
mod auth;

/// The main router
pub async fn main() -> Router {
    let context = Arc::new(HttpContext::init().await);
    Router::new()
        .nest("/health", health::router())
        .nest("/auth", auth::router())
        .fallback(fallback::handler_404)
        .layer(cors())
        .layer(Extension(context))
}

/// CORS layer.
/// It has no protection currently, but it can be changed later.
fn cors() -> CorsLayer {
    use tower_http::cors::Any;
    CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
}
