use axum::{Router, routing::get};
mod get_my_user;
mod edit_my_user;
mod delete_my_user;
mod avatar;

use get_my_user::get_my_user;
use edit_my_user::edit_my_user;
use delete_my_user::delete_my_user;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_my_user)
            .put(edit_my_user)
            .delete(delete_my_user)
        )
        .nest(
            "/avatar",
            avatar::router()
        )
}