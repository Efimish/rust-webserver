use std::sync::Arc;

use crate::http::{AppState, HttpError, HttpResult, Timestampz};
use anyhow::Context;
use axum::{body::Body, extract::Path as ExPath, http::header, response::IntoResponse, Extension};
use serde::Serialize;
// use tokio_util::io::ReaderStream;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Upload {
    upload_id: Uuid,
    file_name: String,
    extension: String,
    content_type: String,
    folder: String,
    size: i64,
    created_at: Timestampz
}

pub async fn get_single_upload(
    Extension(state): Extension<Arc<AppState>>,
    ExPath(upload_id): ExPath<Uuid>
) -> HttpResult<impl IntoResponse> {
    if sqlx::query!(
        r#"
        SELECT COUNT(1) FROM upload
        WHERE upload_id = $1
        "#,
        upload_id
    )
    .fetch_one(&state.pool)
    .await?.count != Some(1) {
        return Err(HttpError::BadRequest);
    }
    
    let upload = sqlx::query_as!(
        Upload,
        r#"
        SELECT *
        FROM upload
        WHERE upload_id = $1
        "#,
        upload_id
    )
    .fetch_one(&state.pool)
    .await?;

    // let file = tokio::fs::File::open(&upload.file_path).await.context("Can not open file")?;
    let file_path = std::env::current_dir().expect("Can not access current directory")
        .join("uploads")
        .join(&upload.folder)
        .join(upload.upload_id.to_string() + &upload.extension);
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
    // let stream = ReaderStream::new(file);
    // let body = Body::from_stream(stream);
    Ok((headers, body))
}