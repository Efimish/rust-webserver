use std::sync::Arc;
use serde::{Serialize, Deserialize};
use axum::{Router, Extension, Json, http::StatusCode, routing::{get, post}};
use uuid::Uuid;
use time::OffsetDateTime;
use crate::{AppState, utils::AuthUser};

pub fn router() -> Router {
    Router::new()
        .route("/user", get(get_my_user))
        .route("/sessions", get(get_my_sessions))
        .route("/sessions/end", post(end_session))
}

#[derive(Serialize)]
struct User {
    #[serde(rename = "userId")]
    user_id: Uuid,
    username: String,
    email: String,
    #[serde(rename = "passwordHash")]
    password_hash: String,
    #[serde(rename = "createdAt")]
    created_at: time::OffsetDateTime,
    #[serde(rename = "updatedAt")]
    updated_at: Option<time::OffsetDateTime>
}

async fn get_my_user(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> Json<User> {
    let user = sqlx::query_as!(User,
        r#"SELECT * FROM "user" WHERE user_id = $1"#,
        user.user_id
    )
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Json(user)
}

#[derive(Serialize)]
struct Session {
    #[serde(rename = "userId")]
    user_id: Uuid,
    #[serde(rename = "sessionId")]
    session_id: Uuid,
    #[serde(rename = "userIp")]
    user_ip: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
    #[serde(rename = "userLocation")]
    user_location: String,
    #[serde(rename = "lastActive")]
    last_active: OffsetDateTime
}

async fn get_my_sessions(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> Json<Vec<Session>> {
    let sessions = sqlx::query_as!(
        Session,
        r#"SELECT * FROM user_session WHERE user_id = $1
        ORDER BY last_active DESC"#, user.user_id
    ).fetch_all(&state.pool).await.unwrap();
    Json(sessions)
}


#[derive(Deserialize)]
struct EndBody {
    session_id: Uuid
}

async fn end_session(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
    body: Json<EndBody>
) -> StatusCode {
    sqlx::query!(
        r#"DELETE FROM user_session WHERE session_id = $1"#, body.session_id
    ).execute(&state.pool).await.unwrap();
    StatusCode::OK
}