use serde::{Serialize, Deserialize};
use uuid::Uuid;

// #[derive(Serialize)]
// #[derive(sqlx::Type)]
// #[sqlx(type_name = "chat_type", rename_all = "lowercase")]
// pub enum ChatType {
//     Saved,
//     Private,
//     Group
// }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DBChat {
    pub chat_id: Uuid,
    pub chat_name: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChat {
    pub chat_name: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindChat {
    pub chat_id: Uuid
}