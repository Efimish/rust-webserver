use axum::{Router, routing::get};
mod get_chat;
mod delete_chat;
mod get_chat_messages;
mod send_message;
mod get_chat_users;
mod add_chat_user;

use get_chat::get_chat;
use delete_chat::delete_chat;
use get_chat_messages::get_chat_messages;
use send_message::send_message;
use get_chat_users::get_chat_users;
use add_chat_user::add_chat_user;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_chat)
            .delete(delete_chat)
        )
        .route(
            "/messages",
            get(get_chat_messages)
            .post(send_message)
        )
        .route(
            "/users",
            get(get_chat_users)
            .post(add_chat_user)
        )
}