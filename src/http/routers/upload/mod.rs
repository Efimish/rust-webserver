use axum::{extract::DefaultBodyLimit, routing::{get, post, Router}};
mod upload_file;
mod upload_avatar;
mod get_single_upload;
use upload_file::upload_file;
use upload_avatar::upload_avatar;
use get_single_upload::get_single_upload;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(upload_file)
            .layer(DefaultBodyLimit::max(1024_usize.pow(3)))
        )
        .route(
            "/avatar",
            post(upload_avatar)
            .layer(DefaultBodyLimit::max(1024_usize.pow(3)))
        )
        .route(
            "/:upload_id",
            get(get_single_upload)
        )
}