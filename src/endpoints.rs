mod auth;
mod me;
mod users;
mod test;

pub use auth::router as auth_router;
pub use me::router as me_router;
pub use users::router as users_router;
pub use test::router as test_router;