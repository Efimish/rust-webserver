use std::sync::Arc;

use axum::{Extension, Json, extract::Path};
use serde::Deserialize;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser, HttpError};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageBody {
    pub context: String
}

pub async fn send_message(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    Path(chat_id): Path<Uuid>,
    body: Json<CreateMessageBody>
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
        INSERT INTO message (
            chat_id, sender_id, context
        ) VALUES (
            $1, $2, $3
        )
        "#,
        chat_id, user.user_id, body.context
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}