use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChatBody {
    pub name: String
}

pub async fn create_chat(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    body: Json<CreateChatBody>
) -> HttpResult<()> {
    let chat_id = sqlx::query!(
        r#"
        INSERT INTO chat (
            name
        ) VALUES (
            $1
        ) RETURNING id
        "#,
        body.name
    )
    .fetch_one(&state.pool)
    .await?
    .id;

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