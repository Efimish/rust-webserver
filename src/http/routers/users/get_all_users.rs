use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub status: Option<String>
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = OK, description = "Returns all users", body = [User])
    ),
    tag = "users"
)]
pub async fn get_all_users(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser
) -> HttpResult<Json<Vec<User>>> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, username, display_name, status
        FROM "user"
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(users))
}