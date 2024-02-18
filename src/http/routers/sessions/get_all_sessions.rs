use std::sync::Arc;
use axum::{Extension, Json};
use crate::http::{models::session::Session, HttpResult, AppState, AuthUser};

pub async fn get_all_sessions(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> HttpResult<Json<Vec<Session>>> {
    let sessions = Session::get_all(&state.pool, user).await?;
    Ok(Json(sessions))
}