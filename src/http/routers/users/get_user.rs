use std::sync::Arc;

use axum::{Extension, Json, extract::Path};
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
    path = "/users/{user_id}",
    responses(
        (status = OK, description = "Returns one user", body = User)
    ),
    tag = "users"
)]
pub async fn get_user(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
    Path(user_id): Path<Uuid>
) -> HttpResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, username, display_name, status
        FROM "user"
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(user))
}