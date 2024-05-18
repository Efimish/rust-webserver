use std::time::Instant;
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use reqwest::Client;
use crate::{http::HttpContext, models::http_models::{ServiceHealth, Health}};

pub async fn check_health(
    ctx: &HttpContext
) -> Health {
    let postgres = check_postgres_health(&ctx.pool).await;
    let redis = check_redis_health(&mut *ctx.redis.lock().await).await;
    let third_party = check_ip_api_health(&ctx.client).await;

    let status = postgres.status && redis.status && third_party.status;

    Health {
        status,
        postgres,
        redis,
        third_party,
    }
}

async fn check_postgres_health(
    pool: &PgPool
) -> ServiceHealth {
    let now = Instant::now();
    let status = sqlx::query!("SELECT 1 AS health_check")
        .fetch_one(pool).await.is_ok();

    let ping = status.then_some(now.elapsed().as_millis());

    ServiceHealth { status, ping }
}

async fn check_redis_health(
    pool: &mut ConnectionManager
) -> ServiceHealth {
    let now = Instant::now();

    let status = redis::cmd("PING")
        .query_async::<_, ()>(pool).await.is_ok();

    let ping = status.then_some(now.elapsed().as_millis());

    ServiceHealth { status, ping }
}

async fn check_ip_api_health(
    client: &Client
) -> ServiceHealth {
    let now = Instant::now();

    let status = client.get("https://ip-api.com")
        .send().await
        .is_ok_and(|r| !r.status().is_server_error());

    let ping = status.then_some(now.elapsed().as_millis());

    ServiceHealth { status, ping }
}