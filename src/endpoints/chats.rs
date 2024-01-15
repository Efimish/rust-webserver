use std::sync::Arc;
use axum::{Json, Router, routing::get, Extension};
use crate::AppState;
use crate::services::chat_service::{
    get_my_chats,
    add_chat
};
use crate::utils::AuthUser;
use crate::utils::ReqResult;
use crate::models::chat_model::{
    DBChat,
    NewChat
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_chats).post(create_chat)
        )
}

async fn get_chats(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<Json<Vec<DBChat>>> {
    let chats = get_my_chats(&state.pool, user.user_id).await?;
    Ok(Json(chats))
}

async fn create_chat(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    body: Json<NewChat>
) -> ReqResult<()> {
    Ok(add_chat(&state.pool, user.user_id, body.chat_name.clone()).await?)
}