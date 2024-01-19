use axum::{Router, routing::get};
pub mod get_all_chats;
pub mod create_chat;
pub mod get_chat;
pub mod delete_chat;
pub mod get_chat_messages;
pub mod send_message;

use get_all_chats::get_all_chats;
use create_chat::create_chat;
use get_chat::get_chat;
use delete_chat::delete_chat;
use get_chat_messages::get_chat_messages;
use send_message::send_message;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_chats)
            .post(create_chat)
        )
        .route(
            "/:chat_id",
            get(get_chat)
            .delete(delete_chat)
        )
        .route(
            "/:chat_id/messages",
            get(get_chat_messages)
            .post(send_message)
        )
}