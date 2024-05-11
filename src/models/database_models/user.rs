use serde::Serialize;
use uuid::Uuid;
use crate::models::TimestamptzOption;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar: Option<Uuid>,
    pub status: String,
    pub online: TimestamptzOption
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar: Option<Uuid>,
    pub status: String
}