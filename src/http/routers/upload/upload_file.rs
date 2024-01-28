use std::sync::Arc;

use crate::http::{AppState, HttpResult, helpers::uploads};
use anyhow::Context;
use axum::{extract::Multipart, Extension};

pub async fn upload_file(
    Extension(state): Extension<Arc<AppState>>,
    mut multipart: Multipart
) -> HttpResult<()> {
    while let Some(field) = multipart.next_field().await
        .context("Can not get next field")?
    {
        let file_name = field.file_name().context("File is missing file name")?.to_string();
        let content_type = field.content_type().context("File is missing content type")?.to_string();
        let data = field.bytes().await.context("File is missing bytes")?;
        uploads::upload_file(&state.pool, file_name, content_type, &data).await?;
    }
    Ok(())
}