use serde::{Serialize, Deserialize};
use woothee::parser::Parser;
use crate::utils::ReqResult;
use anyhow::Context;

#[derive(Debug, Serialize)]
pub struct RequestInfo {
    pub ip: String,
    pub os: String,
    pub country: String,
    pub city: String,
}

impl RequestInfo {
    pub async fn get(ip: &str, agent: &str) -> ReqResult<RequestInfo> {
        #[derive(Deserialize)]
        struct Location {
            country: String,
            city: String,
        }
        let url = format!("http://ip-api.com/json/{}", ip);
        let req = reqwest::get(url)
            .await
            .context("Error requesting user location")?;
        let location = req.json::<Location>()
            .await
            .context("Error requesting user location")?;
        let parser = Parser::new();
        let agent = parser.parse(agent)
            .context("Error parsing user agent")?;
        Ok(RequestInfo {
            ip: ip.to_string(),
            os: agent.os.to_string(),
            country: location.country,
            city: location.city
        })
    }
}