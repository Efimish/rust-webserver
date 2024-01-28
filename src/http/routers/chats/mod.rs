use axum::{Router, routing::get};
mod get_all_chats;
mod create_chat;
mod chat;

use get_all_chats::get_all_chats;
use create_chat::create_chat;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_chats)
            .post(create_chat)
        )
        .nest(
            "/:chat_id",
            chat::router()
        )
}