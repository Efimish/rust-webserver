//! # Grabbing info about user's IP address
//! Getting user's country and city based on their IP address.
//! It is done using third-party api (`ip-api.com`), but can be cached later.

use serde::Deserialize;
use reqwest::Client;
use anyhow::Context;
use crate::http::HttpResult;

#[derive(Deserialize)]
pub struct IpInfo {
    #[serde(rename(deserialize = "query"))]
    pub ip: String,
    pub country: String,
    pub city: String,
}

impl IpInfo {
    pub async fn get(client: &Client, ip: String) -> HttpResult<Self> {
        let url = format!("http://ip-api.com/json/{ip}?fields=66846719");
        let req = client.get(url)
            .send()
            .await
            .context("failed to request user location")?;
        let ip_info: IpInfo = req.json()
            .await
            .context("failed to parse user location")?;
        Ok(ip_info)
    }
}