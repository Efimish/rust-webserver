use std::sync::Arc;
use axum::{Router, Extension, Json, routing::{get, post}, extract::Path};
use uuid::Uuid;
use crate::{
    AppState,
    utils::{AuthUser, ReqResult},
    models::{
        user_model::FullUser,
        session_model::FullSession
    }
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(get_my_user)
        )
        .route(
            "/sessions",
            get(get_my_sessions)
        )
        .route(
            "/sessions/end/:id",
            post(end_session)
        )
        .route(
            "/sessions/endAll",
            post(end_all_sessions)
        )
}

async fn get_my_user(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<Json<FullUser>> {
    let user = sqlx::query_as!(
        FullUser,
        r#"
        SELECT username, email, display_name, status
        FROM "user"
        WHERE user_id = $1
        "#,
        user.user_id
    )
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(user))
}

async fn get_my_sessions(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<Json<Vec<FullSession>>> {
    let sessions: Vec<FullSession> = sqlx::query_as!(
        FullSession,
        r#"
        SELECT *
        FROM user_session
        WHERE user_id = $1
        ORDER BY last_active DESC
        "#,
        user.user_id
    )
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(sessions))
}

async fn end_session(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    _: AuthUser
) -> ReqResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM user_session
        WHERE session_id = $1
        "#,
        id
    ).execute(&state.pool).await?;
    Ok(())
}

async fn end_all_sessions(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM user_session
        WHERE user_id = $1
        "#,
        user.user_id
    ).execute(&state.pool).await?;
    Ok(())
}