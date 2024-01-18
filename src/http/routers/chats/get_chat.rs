use std::sync::Arc;

use axum::{Extension, Json, extract::Path};
use serde::Serialize;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub chat_id: Uuid,
    pub chat_name: String
}

pub async fn get_chat(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    Path(chat_id): Path<Uuid>
) -> HttpResult<Json<Chat>> {
    let chat = sqlx::query_as!(
        Chat,
        r#"
        SELECT c.* FROM chat c
        JOIN chat_user cu ON cu.chat_id = c.chat_id
        WHERE cu.user_id = $1
        AND c.chat_id = $2
        "#,
        user.user_id,
        chat_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(chat))
}