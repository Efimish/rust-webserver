use axum::{Router, routing::{get, post}};
mod get_all_sessions;
mod end_session;
mod end_all_sessions;
use get_all_sessions::get_all_sessions;
use end_session::end_session;
use end_all_sessions::end_all_sessions;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_sessions)
        )
        .route(
            "/end",
            post(end_session)
        )
        .route(
            "/endAll",
            post(end_all_sessions)
        )
}