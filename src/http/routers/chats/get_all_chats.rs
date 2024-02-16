use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Serialize, sqlx::Type)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
pub enum ChatType {
    Saved,
    Private,
    Group
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: Uuid,
    pub r#type: ChatType,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<Uuid>
}

pub async fn get_all_chats(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> HttpResult<Json<Vec<Chat>>> {
    let users = sqlx::query_as!(
        Chat,
        r#"
        SELECT
            c.id,
            c.type "type!: ChatType",
            c.name,
            c.description,
            c.image
        FROM chat c
        JOIN chat_user cu
        ON cu.chat_id = c.id
        WHERE cu.user_id = $1
        "#,
        user.user_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(users))
}