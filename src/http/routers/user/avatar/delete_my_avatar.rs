use std::sync::Arc;

use axum::Extension;

use crate::http::{AppState, AuthUser, HttpError, HttpResult};

pub async fn delete_my_avatar(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
) -> HttpResult<()> {
    let avatar = sqlx::query!(
        r#"
        SELECT avatar
        FROM "user"
        WHERE user_id = $1
        "#,
        user.user_id
    )
    .fetch_one(&state.pool)
    .await?
    .avatar;
    
    if avatar.is_none() {
        return Err(HttpError::BadRequest);
    }

    sqlx::query!(
        r#"
        DELETE FROM upload
        WHERE upload_id = $1
        "#,
        avatar
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}