#![allow(unused)]
use std::net::SocketAddr;
use axum::{Router, http::HeaderMap, Json, routing::get, extract::{ConnectInfo, Path}};
use crate::{AppState, utils::{RequestInfo, Error}};

pub fn router() -> Router {
    Router::new()
        .route("/info", get(ip_and_agent))
}

async fn ip_and_agent(
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    headers: HeaderMap
) -> Result<Json<RequestInfo>, Error> {
    let ip = connect_info.to_string();
    let ip = ip.split_once(":").unwrap().0;
    let agent = headers["user-agent"].to_str().unwrap();
    let info = RequestInfo::get(ip, agent).await?;

    Ok(Json(info))
}