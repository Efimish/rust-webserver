use std::sync::Arc;
use axum::{Router, routing::get, Json, Extension};
use crate::{logic::health, models::http_models::Health};

use crate::http::HttpContext;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(check_health)
        )
}

async fn check_health(
    Extension(ctx): Extension<Arc<HttpContext>>
) -> Json<Health> {
    Json(health::check_health(&ctx).await)
}