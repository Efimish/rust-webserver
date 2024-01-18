use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChatBody {
    pub chat_name: String
}

pub async fn create_chat(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    body: Json<CreateChatBody>
) -> HttpResult<()> {
    let chat_id = sqlx::query!(
        r#"
        INSERT INTO chat (
            chat_name
        ) VALUES (
            $1
        ) RETURNING chat_id
        "#,
        body.chat_name
    )
    .fetch_one(&state.pool)
    .await?
    .chat_id;

    sqlx::query!(
        r#"
        INSERT INTO chat_user (
            user_id, chat_id
        ) VALUES (
            $1, $2
        )
        "#,
        user.user_id,
        chat_id
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}