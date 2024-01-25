use std::sync::Arc;

use crate::http::{AppState, HttpResult, Timestampz};
use anyhow::Context;
use axum::{body::Body, extract::Path as ExPath, http::header, response::IntoResponse, Extension};
use serde::Serialize;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Upload {
    upload_id: Uuid,
    file_name: String,
    file_path: String,
    content_type: String,
    size: i64,
    created_at: Timestampz
}

pub async fn get_single_upload(
    Extension(state): Extension<Arc<AppState>>,
    ExPath(upload_id): ExPath<Uuid>
) -> HttpResult<impl IntoResponse> {
    let upload = sqlx::query_as!(
        Upload,
        r#"
        SELECT * FROM upload
        WHERE upload_id = $1
        "#,
        upload_id
    )
    .fetch_one(&state.pool)
    .await?;

    let file = tokio::fs::File::open(&upload.file_path).await.context("Can not open file")?;
    let headers = [
        // (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
        (
            header::CONTENT_TYPE,
            format!("{}; charset=utf-8", &upload.content_type)
        ),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename={}", &upload.file_name),
        ),
    ];
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    Ok((headers, body))
}