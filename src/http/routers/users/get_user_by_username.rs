use std::sync::Arc;

use axum::{Extension, extract::Path, Json};
use super::get_user::User;

use crate::http::{HttpResult, AppState, AuthUser};

pub async fn get_user_by_username(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
    Path(username): Path<String>
) -> HttpResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT u.user_id, u.username, u.display_name, u.status,
        (
            select max(last_active)
            from user_session us
            where us.user_id = u.user_id
        ) online
        FROM "user" u
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(user))
}