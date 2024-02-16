use std::sync::Arc;

use axum::{Extension, extract::Path, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::http::{AppState, AuthUser, HttpResult, HttpError};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub status: Option<String>
}

pub async fn get_chat_users(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    Path(chat_id): Path<Uuid>
) -> HttpResult<Json<Vec<User>>> {
    if sqlx::query!(
        r#"
        SELECT COUNT(1) FROM chat_user
        WHERE user_id = $1
        AND chat_id = $2
        "#,
        user.user_id,
        chat_id
    )
    .fetch_one(&state.pool)
    .await?
    .count != Some(1) {
        return Err(HttpError::BadRequest);
    }
    
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT
            u.id,
            u.username,
            u.display_name,
            u.status
        FROM "user" u
        JOIN chat_user cu on cu.user_id = u.id
        WHERE cu.chat_id = $1
        "#,
        chat_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(users))
}