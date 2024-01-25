use std::{ffi::OsStr, fs, io::Write, path::Path, sync::Arc};

use crate::http::{AppState, HttpResult};
use anyhow::Context;
use axum::{extract::Multipart, Extension};
use uuid::Uuid;

pub async fn upload_file(
    Extension(state): Extension<Arc<AppState>>,
    mut multipart: Multipart
) -> HttpResult<()> {
    while let Some(field) = multipart.next_field().await
        .context("Can not get next field")?
    {
        let name = field.name().context("File is missing name")?.to_string();
        let original_file_name = field.file_name().context("File is missing file name")?.to_string();
        let content_type = field.content_type().context("File is missing content type")?.to_string();
        let data = field.bytes().await.context("File is missing bytes")?;
        let size = data.len() as i64;

        let upload_id = Uuid::new_v4();
        let extension = Path::new(&original_file_name).extension().and_then(OsStr::to_str).context("File is missing extension")?;
        let file_name = format!("{}.{}", upload_id, extension);
        let file_path = std::env::current_dir().expect("Can not access current directory").join("uploads").join(&file_name);
        // log::info!("File path: {:?}", file_path);
        let mut file = fs::File::create(&file_path).context("Can not create file")?;
        file.write_all(&data).context("Can not write data to file")?;
        let file_path = file_path.to_str().context("Not a valid UTF-8 file path")?;

        sqlx::query!(
            r#"
            INSERT INTO upload (
                upload_id,
                file_name,
                file_path,
                content_type,
                size
            ) VALUES (
                $1, $2, $3, $4, $5
            )
            "#,
            upload_id,
            original_file_name,
            file_path,
            content_type,
            size
        )
        .execute(&state.pool)
        .await?;

        log::info!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    }
    Ok(())
}