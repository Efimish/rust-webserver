use std::sync::Arc;
use axum::extract::Path;
use axum::{Json, Router, routing::{get, post}, Extension};
use uuid::Uuid;
use crate::AppState;
use crate::models::message_model::{DBMessage, NewMessage};
use crate::services::chat_service::{
    get_chat_messages,
    send_chat_message
};
use crate::utils::AuthUser;
use crate::utils::ReqResult;

pub fn router() -> Router {
    Router::new()
        .route(
            "/:chat_id",
            get(get_messages)
        )
        .route(
            "/",
            post(send_message)
        )
}

async fn get_messages(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    Path(chat_id): Path<Uuid>
) -> ReqResult<Json<Vec<DBMessage>>> {
    let messages = get_chat_messages(&state.pool, user.user_id, chat_id).await?;
    Ok(Json(messages))
}

async fn send_message(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    body: Json<NewMessage>,
) -> ReqResult<()> {
    send_chat_message(&state.pool, user.user_id, body.chat_id, body.context.clone()).await?;
    Ok(())
}