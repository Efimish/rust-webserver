use serde::Serialize;
use uuid::Uuid;
use crate::models::Timestamptz;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_ip: String,
    pub user_agent: String,
    pub user_country: String,
    pub user_city: String,
    pub last_active: Timestamptz
}