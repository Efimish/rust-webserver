use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::Timestampz;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DBMessage {
    pub message_id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub context: String,
    pub created_at: Timestampz,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMessage {
    pub chat_id: Uuid,
    pub context: String,
}