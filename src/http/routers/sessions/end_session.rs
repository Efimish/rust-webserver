use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndSessionBody {
    session_id: Uuid
}

pub async fn end_session(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    body: Json<EndSessionBody>
) -> HttpResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM user_session
        WHERE id = $1
        AND user_id = $2
        "#,
        body.session_id,
        user.user_id
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}