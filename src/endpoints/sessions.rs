use std::sync::Arc;
use axum::{Router, Extension, Json, routing::{get, post}, extract::Path};
use uuid::Uuid;
use crate::{
    AppState,
    utils::{AuthUser, ReqResult},
    models::session_model::FullSession,
    services::auth_service::{
        get_user_sessions,
        remove_user_session,
        remove_all_user_sessions
    }
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_sessions)
        )
        .route(
            "/end/:id",
            post(end_session)
        )
        .route(
            "/endAll",
            post(end_all_sessions)
        )
}

async fn get_all_sessions(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<Json<Vec<FullSession>>> {
    let sessions = get_user_sessions(&state.pool, user.user_id).await?;
    Ok(Json(sessions))
}

async fn end_session(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    Path(session_id): Path<Uuid>
) -> ReqResult<()> {
    remove_user_session(&state.pool, user.user_id, session_id).await?;
    Ok(())
}

async fn end_all_sessions(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<()> {
    remove_all_user_sessions(&state.pool, user.user_id).await?;
    Ok(())
}