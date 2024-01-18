use std::{sync::Arc, time::Instant};
use axum::{Router, routing::get, Json, Extension};
use reqwest::Client;
use serde::Serialize;
use sqlx::PgPool;
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
struct DatabaseHealth {
    status: bool,
    ping: Option<f32>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThirdPartyHealth {
    status: bool,
    ping: Option<f32>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Health {
    status: bool,
    database: DatabaseHealth,
    third_party: ThirdPartyHealth,
}

async fn check_health(
    Extension(state): Extension<Arc<AppState>>
) -> HttpResult<Json<Health>> {
    let database = test_database(&state.pool).await;
    let third_party = test_ip_api(&state.client).await;
    let status = database.status && third_party.status;
    let health = Health {
        status,
        database,
        third_party,
    };
    Ok(Json(health))
}

async fn test_database(
    pool: &PgPool
) -> DatabaseHealth {
    let now = Instant::now();
    if sqlx::query!(r#"SELECT COUNT(1)"#)
    .fetch_one(pool).await.is_ok() {
        DatabaseHealth {
            status: true,
            ping: Some(now.elapsed().as_secs_f32())
        }
    } else {
        DatabaseHealth {
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
            ping: Some(now.elapsed().as_secs_f32()),
        }
    } else {
        ThirdPartyHealth {
            status: false,
            ping: None
        }
    }
}