use serde::Serialize;
use uuid::Uuid;
use super::Timestampz;

#[derive(Serialize)]
struct Session {
    #[serde(rename = "userId")]
    user_id: Uuid,
    #[serde(rename = "sessionId")]
    session_id: Uuid,
    #[serde(rename = "userIp")]
    user_ip: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
    #[serde(rename = "userLocation")]
    user_location: String,
    #[serde(rename = "lastActive")]
    last_active: Timestampz
}