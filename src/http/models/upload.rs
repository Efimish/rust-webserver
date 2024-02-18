use serde::Serialize;
use uuid::Uuid;
use super::Timestampz;

use crate::http::{HttpResult, HttpError, HttpContext};
use sqlx::PgPool;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub file_name: String,
    pub extension: String,
    pub content_type: String,
    pub folder: String,
    pub size: i64,
    pub created_at: Timestampz
}

impl Upload {
    pub async fn get(
        pool: &PgPool,
        id: Uuid
    ) -> HttpResult<Self> {
        let upload = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM upload
            WHERE id = $1
            "#, id
        )
        .fetch_optional(pool)
        .await?
        .http_context(HttpError::NotFound)?;

        Ok(upload)
    }
}