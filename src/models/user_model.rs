use serde::{Serialize, Deserialize};
use validator::Validate;
use regex::Regex;
use lazy_static::lazy_static;
use uuid::Uuid;
use super::Timestampz;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub status: Option<String>,
    pub created_at: Timestampz,
    pub updated_at: Timestampz
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseUser {
    pub username: String,
    pub display_name: String,
    pub status: Option<String>
}

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"^\w+$").unwrap();
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBody {
    #[validate(
        length(
            min = 3,
            max = 24,
            message = "Username must be between 3 and 24 characters"
        ),
        regex(
            path = "USERNAME_REGEX",
            message = "Username must only contain english letters, numbers and unserscore"
        )
    )]
    pub username: String,
    #[validate(
        email(
            message = "Email must be valid"
        )
    )]
    pub email: String,
    #[validate(
        length(
            min = 3,
            message = "Password must be at least 3 characters"
        )
    )]
    pub password: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUser {
    pub username: String,
    pub email: String,
    pub password_hash: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginBody {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshBody {
    pub refresh_token: String
}