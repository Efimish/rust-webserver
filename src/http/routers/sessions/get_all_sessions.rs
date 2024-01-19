use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::http::{HttpResult, AppState, AuthUser, Timestampz};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub user_ip: String,
    pub user_agent: String,
    pub user_country: String,
    pub user_city: String,
    pub last_active: Timestampz
}

#[utoipa::path(
    get,
    path = "/sessions",
    responses(
        (status = OK, description = "Get all your sessions", body = [Session])
    ),
    tag = "sessions"
)]
pub async fn get_all_sessions(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> HttpResult<Json<Vec<Session>>> {
    let sessions = sqlx::query_as!(
        Session,
        r#"
        SELECT * FROM user_session
        WHERE user_id = $1
        ORDER BY last_active DESC
        "#,
        user.user_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(sessions))
}