mod auth;
mod me;
mod users;
mod test;
mod sessions;
mod chats;
mod messages;

pub use auth::router as auth_router;
pub use me::router as me_router;
pub use users::router as users_router;
pub use test::router as test_router;
pub use sessions::router as sessions_router;
pub use chats::router as chats_router;
pub use messages::router as messages_router;