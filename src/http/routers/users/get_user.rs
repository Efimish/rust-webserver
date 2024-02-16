use std::sync::Arc;

use axum::{Extension, Json, extract::Path};
use serde::Serialize;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser, models::TimestampzOption};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar: Option<Uuid>,
    pub status: Option<String>,
    pub online: TimestampzOption
}

pub async fn get_user(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
    Path(user_id): Path<Uuid>
) -> HttpResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT u.id, u.username, u.display_name, u.avatar, u.status,
        (
            select max(last_active)
            from user_session us
            where us.user_id = u.id
        ) online
        FROM "user" u
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(user))
}