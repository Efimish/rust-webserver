use std::{sync::Arc, time::Instant};
use axum::{Router, routing::get, Json, Extension};
use reqwest::Client;
use serde::Serialize;
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use crate::http::HttpResult;

use super::AppState;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(check_health)
        )
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PostgresHealth {
    status: bool,
    ping: Option<u128>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RedisHealth {
    status: bool,
    ping: Option<u128>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThirdPartyHealth {
    status: bool,
    ping: Option<u128>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Health {
    status: bool,
    postgres: PostgresHealth,
    redis: RedisHealth,
    third_party: ThirdPartyHealth,
}

async fn check_health(
    Extension(state): Extension<Arc<AppState>>
) -> HttpResult<Json<Health>> {
    let postgres = test_postgres(&state.pool).await;
    let redis = test_redis(&mut *state.redis.lock().await).await;
    let third_party = test_ip_api(&state.client).await;
    let status = postgres.status && redis.status && third_party.status;
    let health = Health {
        status,
        postgres,
        redis,
        third_party,
    };
    Ok(Json(health))
}

async fn test_postgres(
    pool: &PgPool
) -> PostgresHealth {
    let now = Instant::now();
    if sqlx::query!(r#"SELECT COUNT(1)"#)
    .fetch_one(pool).await.is_ok() {
        PostgresHealth {
            status: true,
            ping: Some(now.elapsed().as_millis())
        }
    } else {
        PostgresHealth {
            status: false,
            ping: None
        }
    }
}

async fn test_redis(
    pool: &mut ConnectionManager
) -> RedisHealth {
    let now = Instant::now();
    if redis::cmd("PING")
    .query_async::<_,()>(pool).await.is_ok() {
        RedisHealth {
            status: true,
            ping: Some(now.elapsed().as_millis())
        }
    } else {
        RedisHealth {
            status: false,
            ping: None
        }
    }
}

async fn test_ip_api(
    client: &Client
) -> ThirdPartyHealth {
    let now = Instant::now();
    if client.get("https://ip-api.com")
    .send()
    .await.is_ok() {
        ThirdPartyHealth {
            status: true,
            ping: Some(now.elapsed().as_millis()),
        }
    } else {
        ThirdPartyHealth {
            status: false,
            ping: None
        }
    }
}