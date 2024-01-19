use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndSessionBody {
    session_id: Uuid
}

#[utoipa::path(
    post,
    path = "/sessions/end",
    request_body = EndSessionBody,
    responses(
        (status = OK, description = "End just one session")
    ),
    tag = "sessions"
)]
pub async fn end_session(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    body: Json<EndSessionBody>
) -> HttpResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM user_session
        WHERE user_id = $1
        AND session_id = $2
        "#,
        user.user_id,
        body.session_id
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}