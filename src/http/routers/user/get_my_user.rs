use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub status: Option<String>
}

pub async fn get_my_user(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
) -> HttpResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, username, email, display_name, status
        FROM "user"
        WHERE user_id = $1
        "#,
        user.user_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(user))
}