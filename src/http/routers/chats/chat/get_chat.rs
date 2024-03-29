use std::sync::Arc;

use axum::{Extension, Json, extract::Path};
use uuid::Uuid;
use super::super::get_all_chats::{Chat, ChatType};

use crate::http::{HttpResult, AppState, AuthUser};

pub async fn get_chat(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    Path(chat_id): Path<Uuid>
) -> HttpResult<Json<Chat>> {
    let chat = sqlx::query_as!(
        Chat,
        r#"
        SELECT
            c.id,
            c.type "type: ChatType",
            c.name,
            c.description,
            c.image
        FROM chat c
        JOIN chat_user cu ON cu.chat_id = c.id
        WHERE cu.user_id = $1
        AND c.id = $2
        "#,
        user.user_id,
        chat_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(chat))
}