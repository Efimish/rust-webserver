use std::sync::Arc;

use axum::{Extension, extract::Path, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::http::{AppState, AuthUser, HttpResult, HttpError};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUserBody {
    pub user_id: Uuid
}

pub async fn add_chat_user(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    Path(chat_id): Path<Uuid>,
    body: Json<AddUserBody>
) -> HttpResult<()> {
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
    
    sqlx::query!(
        r#"
        INSERT INTO chat_user (
            user_id, chat_id
        ) VALUES (
            $1, $2
        )
        "#,
        body.user_id,
        chat_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(())
}