use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::Timestampz;

#[derive(Serialize, Deserialize)]
pub struct BaseUser {
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub username: String
}

#[derive(Serialize)]
pub struct FullUser {
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    #[serde(rename = "createdAt")]
    pub created_at: Timestampz,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<Timestampz>
}