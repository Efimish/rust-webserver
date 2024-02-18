use serde::Serialize;
use uuid::Uuid;
use super::Timestampz;

use crate::http::{AuthUser, HttpResult};
use sqlx::PgPool;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_ip: String,
    pub user_agent: String,
    pub user_country: String,
    pub user_city: String,
    pub last_active: Timestampz,
    pub current: bool
}

impl Session {
    pub async fn get_all(
        pool: &PgPool,
        user: AuthUser
    ) -> HttpResult<Vec<Self>> {
        let sessions = sqlx::query!(
            r#"
            SELECT *
            FROM user_session
            WHERE user_id = $1
            ORDER BY last_active DESC
            "#,
            user.user_id,
        )
        .fetch_all(pool)
        .await?;

        let sessions: Vec<Self> = sessions.into_iter().map(|s| {
            Self {
                id: s.id,
                user_id: s.user_id,
                user_ip: s.user_ip,
                user_agent: s.user_agent,
                user_country: s.user_country,
                user_city: s.user_city,
                last_active: Timestampz(s.last_active),
                current: s.id == user.session_id
            }
        }).collect();

        Ok(sessions)
    }
}