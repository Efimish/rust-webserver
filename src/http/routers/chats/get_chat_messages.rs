use std::sync::Arc;

use axum::{Extension, Json, extract::Path};
use serde::Serialize;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser, HttpError, Timestampz};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message_id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub reply_message_id: Option<Uuid>,
    pub forward_message_id: Option<Uuid>,
    pub context: Option<String>,
    pub edited: bool,
    pub created_at: Timestampz,
    pub updated_at: Timestampz
}

pub async fn get_chat_messages(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    Path(chat_id): Path<Uuid>
) -> HttpResult<Json<Vec<Message>>> {
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

    let messages = sqlx::query_as!(
        Message,
        r#"
        SELECT
            message_id,
            chat_id,
            sender_id,
            reply_message_id,
            forward_message_id,
            context,
            edited,
            created_at,
            updated_at
        FROM message
        WHERE chat_id = $1
        "#,
        chat_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(messages))
}