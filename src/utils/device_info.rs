use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use woothee::parser::Parser;
use anyhow::Context;
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, ConnectInfo},
    http::{request::Parts, header::USER_AGENT}
};
use crate::{utils::{Error, ReqResult}, models::session_model::NewSession};

#[derive(Debug, Serialize)]
pub struct DeviceInfo {
    pub ip: String,
    pub os: String,
    pub country: String,
    pub city: String,
}

impl DeviceInfo {
    pub async fn get(
        ip: &str,
        agent: &str
    ) -> ReqResult<Self> {
        #[derive(Deserialize)]
        struct Location {
            country: String,
            city: String,
        }
        let url = format!("http://ip-api.com/json/{}", ip);
        let req = reqwest::get(url)
            .await
            .context("Error requesting user location")?;
        let location: Location = req.json()
            .await
            .context("Error requesting user location")?;
        let parser = Parser::new();
        let agent = parser.parse(agent)
            .context("Error parsing user agent")?;
        Ok(Self {
            ip: ip.to_string(),
            os: agent.os.to_string(),
            country: location.country,
            city: location.city
        })
    }
    pub fn to_session(self, user_id: Uuid) -> NewSession {
        NewSession {
            user_id,
            user_ip: self.ip,
            user_agent: self.os,
            user_country: self.country,
            user_city: self.city,
        }
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for DeviceInfo
where
    B: Send + Sync
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
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

        DeviceInfo::get(user_ip, user_agent).await
    }
}