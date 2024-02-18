use std::sync::Arc;
use axum::{body::Body, extract::Path, http::header, response::IntoResponse, Extension};
use uuid::Uuid;
use anyhow::Context;
use crate::http::{models::upload::Upload, AppState, HttpResult};

pub async fn get_upload(
    Extension(state): Extension<Arc<AppState>>,
    Path(upload_id): Path<Uuid>
) -> HttpResult<impl IntoResponse> {    
    let upload = Upload::get(&state.pool, upload_id).await?;
    let file_path = std::env::current_dir().expect("Can not access current directory")
        .join("uploads")
        .join(&upload.folder)
        .join(upload.id.to_string() + &upload.extension);
    log::info!("Reading file at: {file_path:?}");
    let bytes = tokio::fs::read(&file_path).await.context("Can not read file")?;
    let body = Body::from(bytes);
    let headers = [
        (
            header::CONTENT_TYPE,
            format!("{}; charset=utf-8", &upload.content_type)
        ),
        (
            header::CONTENT_DISPOSITION,
            format!("inline; filename={}", &upload.file_name),
        ),
    ];
    Ok((headers, body))
}