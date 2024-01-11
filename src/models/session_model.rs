use serde::Serialize;
use uuid::Uuid;
use super::Timestampz;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullSession {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub user_ip: String,
    pub user_agent: String,
    pub user_country: String,
    pub user_city: String,
    pub last_active: Timestampz
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSession {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub user_ip: String,
    pub user_agent: String,
    pub user_country: String,
    pub user_city: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSession {
    pub user_id: Uuid,
    pub user_ip: String,
    pub user_agent: String,
    pub user_country: String,
    pub user_city: String,
}

impl NewSession {
    pub fn to_base(self, session_id: Uuid) -> BaseSession {
        BaseSession {
            user_id: self.user_id,
            session_id,
            user_ip: self.user_ip,
            user_agent: self.user_agent,
            user_country: self.user_country,
            user_city: self.user_city
        }
    }
}