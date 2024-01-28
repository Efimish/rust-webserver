use axum::{Router, routing::put};
mod edit_my_avatar;
mod delete_my_avatar;

use edit_my_avatar::edit_my_avatar;
use delete_my_avatar::delete_my_avatar;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            put(edit_my_avatar)
            .delete(delete_my_avatar)
        )
}