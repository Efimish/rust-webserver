use std::sync::Arc;

use crate::http::{models::Timestampz, AppState, HttpContext, HttpError, HttpResult};
use anyhow::Context;
use axum::{body::Body, extract::Path as ExPath, http::header, response::IntoResponse, Extension};
use serde::Serialize;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Upload {
    id: Uuid,
    file_name: String,
    extension: String,
    content_type: String,
    folder: String,
    size: i64,
    created_at: Timestampz
}

pub async fn stream_upload(
    Extension(state): Extension<Arc<AppState>>,
    ExPath(upload_id): ExPath<Uuid>
) -> HttpResult<impl IntoResponse> {    
    let upload = sqlx::query_as!(
        Upload,
        r#"
        SELECT *
        FROM upload
        WHERE id = $1
        "#,
        upload_id
    )
    .fetch_optional(&state.pool)
    .await?
    .http_context(HttpError::NotFound)?;

    let file_path = std::env::current_dir().expect("Can not access current directory")
        .join("uploads")
        .join(&upload.folder)
        .join(upload.id.to_string() + &upload.extension);
    let file = tokio::fs::File::open(&file_path).await.context("Can not open file")?;
    let stream = ReaderStream::new(file);
    log::info!("Streaming file at: {file_path:?}");
    let body = Body::from_stream(stream);
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