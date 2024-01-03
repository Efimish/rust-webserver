use serde::{Serialize, Deserialize};
use woothee::parser::Parser;
use crate::utils::Error;
use anyhow::anyhow;

#[derive(Debug, Serialize)]
pub struct RequestInfo {
    pub ip: String,
    pub os: String,
    pub country: String,
    pub city: String,
}

impl RequestInfo {
    pub async fn get(ip: &str, agent: &str) -> Result<RequestInfo, Error> {
        #[derive(Deserialize)]
        struct Location {
            country: String,
            city: String,
        }
        let url = format!("http://ip-api.com/json/{}", ip);
        let req = reqwest::get(url)
            .await
            .map_err(|_| {
                Error::Anyhow(
                    anyhow!("Can not request user location")
                )
            })?;
        let location = req.json::<Location>()
            .await
            .map_err(|_| {
                Error::Anyhow(
                    anyhow!("Can not parse user location")
                )
            })?;
        let parser = Parser::new();
        let agent = parser.parse(agent)
            .ok_or(Error::Anyhow(
                anyhow!("Can not parse user agent")
            ))?;
        Ok(RequestInfo {
            ip: ip.to_string(),
            os: agent.os.to_string(),
            country: location.country,
            city: location.city
        })
    }
}