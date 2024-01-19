use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub chat_id: Uuid,
    pub chat_name: String
}

#[utoipa::path(
    get,
    path = "/chats",
    responses(
        (status = OK, description = "Get all your chats", body = [Chat])
    ),
    tag = "chats"
)]
pub async fn get_all_chats(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> HttpResult<Json<Vec<Chat>>> {
    let users = sqlx::query_as!(
        Chat,
        r#"
        SELECT c.* FROM chat c
        JOIN chat_user cu ON cu.chat_id = c.chat_id
        WHERE cu.user_id = $1
        "#,
        user.user_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(users))
}