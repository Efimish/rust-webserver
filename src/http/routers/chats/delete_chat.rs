use std::sync::Arc;

use axum::{Extension, extract::Path};
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser};

pub async fn delete_chat(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
    Path(chat_id): Path<Uuid>
) -> HttpResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM chat
        WHERE chat_id = $1
        "#,
        chat_id
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}