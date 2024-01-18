use std::sync::Arc;

use axum::Extension;

use crate::http::{HttpResult, AppState, AuthUser};

pub async fn delete_my_user(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
) -> HttpResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM "user"
        WHERE user_id = $1
        "#,
        user.user_id
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}