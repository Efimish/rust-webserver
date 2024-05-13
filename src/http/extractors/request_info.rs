//! Extractor of user device info.

use crate::{
    http::{HttpError, HttpResult},
    utils::ip_info::IpInfo,
    utils::user_agent::parse_user_agent,
};
use anyhow::Context;
use async_trait::async_trait;
use axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::{header::USER_AGENT, request::Parts},
};
use reqwest::Client;
use std::net::SocketAddr;

/// # Info about request
/// ip address and user agent.
pub struct RequestInfo {
    pub ip: String,
    pub agent: String,
}

/// # Info about request and it's location
/// country and city are based on ip.
pub struct RequestInfoWithLocation {
    pub ip: String,
    pub agent: String,
    pub country: String,
    pub city: String,
}

impl RequestInfo {
    pub async fn fetch_location(self, client: &Client) -> HttpResult<RequestInfoWithLocation> {
        let ip_info = IpInfo::get(client, self.ip).await?;
        Ok(RequestInfoWithLocation {
            ip: ip_info.ip,
            agent: self.agent,
            country: ip_info.country,
            city: ip_info.city,
        })
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for RequestInfo
where
    B: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
        let connect_info: ConnectInfo<SocketAddr> = ConnectInfo::from_request_parts(req, _s)
            .await
            .context("failed to get connect info from request")?;

        let ip = req
            .headers
            .get("x-real-ip")
            .map(|header| header.to_str().ok())
            .flatten()
            .map(|header| header.to_string())
            .unwrap_or(connect_info.ip().to_string());

        let user_agent = req
            .headers
            .get(USER_AGENT)
            .context("failed to get user agent")?
            .to_str()
            .context("failed to get user agent")?;

        let agent = parse_user_agent(user_agent);

        Ok(Self { ip, agent })
    }
}
