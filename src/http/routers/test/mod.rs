use std::{collections::HashMap, sync::Arc};
use axum::{extract::Path, routing::{get, post}, Extension, Json, Router};
use redis::AsyncCommands;
use crate::http::HttpResult;

use super::AppState;

pub fn router() -> Router {
    Router::new()
        .route(
            "/redis",
            get(get_redis)
        )
        .route(
            "/redis/:key/:id",
            post(set_redis)
        )
}

async fn get_redis(
    Extension(state): Extension<Arc<AppState>>
) -> HttpResult<Json<HashMap<String, String>>> {
    let keys: Vec<String> = state.redis.lock().await.keys("*").await?;
    let mut result: HashMap<String, String> = HashMap::new();
    for key in keys {
        let value: String = state.redis.lock().await.get(&key).await?;
        result.insert(key, value);
    }
    Ok(Json(result))
}

async fn set_redis(
    Extension(state): Extension<Arc<AppState>>,
    Path((key, value)): Path<(String, String)>
) -> HttpResult<()> {
    state.redis.lock().await.set(key, value).await?;
    Ok(())
}