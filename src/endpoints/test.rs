#![allow(unused)]
use std::net::SocketAddr;
use axum::{Router, http::HeaderMap, Json, routing::get, extract::{ConnectInfo, Path}};
use crate::{AppState, utils::{Error, ReqResult, DeviceInfo}};

pub fn router() -> Router {
    Router::new()
        .route(
            "/info",
            get(ip_and_agent)
        )
}

async fn ip_and_agent(
    info: DeviceInfo
) -> ReqResult<Json<DeviceInfo>> {
    Ok(Json(info))
}