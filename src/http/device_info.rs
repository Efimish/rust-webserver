use std::{net::SocketAddr, sync::Arc};
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use woothee::parser::Parser;
use anyhow::Context;
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, ConnectInfo},
    http::{request::Parts, header::USER_AGENT}, Extension
};
use crate::http::{HttpError, HttpResult};
use super::AppState;

#[derive(Serialize)]
pub struct DeviceInfo {
    pub ip: String,
    pub os: String,
    pub country: String,
    pub city: String,
}

lazy_static! {
    static ref PARSER: Parser = Parser::new();
}

impl DeviceInfo {
    pub async fn get(
        client: &Client,
        ip: &str,
        agent: &str
    ) -> HttpResult<Self> {
        #[derive(Deserialize)]
        struct Location {
            country: String,
            city: String,
        }
        let url = format!("http://ip-api.com/json/{}", ip);
        let req = client.get(url)
            .send()
            .await
            .context("Error requesting user location")?;
        let location: Location = req.json()
            .await
            .context("Error requesting user location")?;
        let agent = PARSER.parse(agent)
            .context("Error parsing user agent")?;
        Ok(Self {
            ip: ip.to_string(),
            os: agent.os.to_string(),
            country: location.country,
            city: location.city
        })
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for DeviceInfo
where
    B: Send + Sync
{
    type Rejection = HttpError;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
        let state: Extension<Arc<AppState>> = Extension::from_request_parts(req, _s)
            .await
            .expect("BUG: AppState was not added as an extension");
        
        let connect_info: ConnectInfo<SocketAddr> = ConnectInfo::from_request_parts(req, _s)
            .await
            .context("Error getting connect info from request")?;

        let user_ip = connect_info.0.to_string();
        let user_ip = user_ip
            .split_once(":")
            .context("Error getting request IP")?.0;
        
        let user_agent = req
            .headers
            .get(USER_AGENT)
            .context("Error getting user agent")?
            .to_str()
            .context("Error getting user agent")?;

        DeviceInfo::get(
            &state.client,
            user_ip,
            user_agent
        ).await
    }
}