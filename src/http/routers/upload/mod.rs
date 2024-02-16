use axum::{extract::DefaultBodyLimit, routing::{get, post, Router}};
mod upload_file;
mod get_upload;
mod stream_upload;
use upload_file::upload_file;
use get_upload::get_upload;
use stream_upload::stream_upload;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(upload_file)
            .layer(DefaultBodyLimit::max(1024_usize.pow(3)))
        )
        .route(
            "/:upload_id",
            get(get_upload)
        )
        .route(
            "/:upload_id/stream",
            get(stream_upload)
        )
}