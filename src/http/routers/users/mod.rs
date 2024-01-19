use axum::{Router, routing::get};
mod get_all_users;
mod get_user;
mod get_user_by_username;

use get_all_users::get_all_users;
use get_user::get_user;
use get_user_by_username::get_user_by_username;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_users)
        )
        .route(
            "/:user_id",
            get(get_user)
        )
        .route(
            "/username/:username",
            get(get_user_by_username)
        )
}