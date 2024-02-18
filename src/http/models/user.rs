use serde::Serialize;
use uuid::Uuid;
use super::TimestampzOption;

use crate::http::{HttpResult, HttpError, HttpContext};
use sqlx::PgPool;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar: Option<Uuid>,
    pub status: Option<String>,
    pub online: TimestampzOption
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar: Option<Uuid>,
    pub status: Option<String>
}

impl User {
    pub async fn get(
        pool: &PgPool,
        id: Uuid
    ) -> HttpResult<Self> {
        let user = sqlx::query_as!(
            Self,
            r#"
            SELECT
                u.id,
                u.username,
                u.display_name,
                u.avatar,
                u.status,
                (
                    select max(last_active)
                    from user_session us
                    where us.user_id = u.id
                ) online
            FROM "user" u
            WHERE u.id = $1
            "#, id
        )
        .fetch_optional(pool)
        .await?
        .http_context(HttpError::NotFound)?;

        Ok(user)
    }

    pub async fn get_by_username(
        pool: &PgPool,
        username: String
    ) -> HttpResult<Self> {
        let user = sqlx::query_as!(
            Self,
            r#"
            SELECT
                u.id,
                u.username,
                u.display_name,
                u.avatar,
                u.status,
                (
                    select max(last_active)
                    from user_session us
                    where us.user_id = u.id
                ) online
            FROM "user" u
            WHERE u.username = $1
            "#, username
        )
        .fetch_optional(pool)
        .await?
        .http_context(HttpError::NotFound)?;

        Ok(user)
    }

    pub async fn get_all(
        pool: &PgPool
    ) -> HttpResult<Vec<Self>> {
        let users = sqlx::query_as!(
            Self,
            r#"
            SELECT
                u.id,
                u.username,
                u.display_name,
                u.avatar,
                u.status,
                (
                    select max(last_active)
                    from user_session us
                    where us.user_id = u.id
                ) online
            FROM "user" u
            "#
        )
        .fetch_all(pool)
        .await?;
        
        Ok(users)
    }
}

impl MyUser {
    pub async fn get(
        pool: &PgPool,
        id: Uuid
    ) -> HttpResult<Self> {
        let user = sqlx::query_as!(
            Self,
            r#"
            SELECT
                id,
                username,
                email,
                display_name,
                avatar,
                status
            FROM "user"
            WHERE id = $1
            "#, id
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }
}