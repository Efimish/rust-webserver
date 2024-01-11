use serde::Serialize;
use uuid::Uuid;
use super::Timestampz;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub user_ip: String,
    pub user_agent: String,
    pub user_location: String,
    pub last_active: Timestampz
}