use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceHealth {
    pub status: bool,
    pub ping: Option<u128>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub status: bool,
    pub postgres: ServiceHealth,
    pub redis: ServiceHealth,
    pub third_party: ServiceHealth,
}