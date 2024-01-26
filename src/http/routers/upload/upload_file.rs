use std::{ffi::OsStr, path::Path, sync::Arc};

use crate::http::{AppState, HttpResult};
use anyhow::Context;
use axum::{extract::Multipart, Extension};
use time::OffsetDateTime;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn upload_file(
    Extension(state): Extension<Arc<AppState>>,
    mut multipart: Multipart
) -> HttpResult<()> {
    while let Some(field) = multipart.next_field().await
        .context("Can not get next field")?
    {
        // let name = field.name().context("File is missing name")?.to_string();
        let original_file_name = field.file_name().context("File is missing file name")?.to_string();
        let content_type = field.content_type().context("File is missing content type")?.to_string();
        let data = field.bytes().await.context("File is missing bytes")?;
        let size = data.len() as i64;

        let upload_id = Uuid::new_v4();
        let extension = Path::new(&original_file_name).extension()
            .and_then(OsStr::to_str)
            .map(|s| format!(".{s}"))
            .unwrap_or_default();
        let file_name = format!("{}{}", upload_id, extension);
        let folder = {
            let date = OffsetDateTime::now_utc();
            let year = date.year();
            let fix_date = |n: u8| format!("{n:02}");
            let month = fix_date(date.month().into());
            let day = fix_date(date.day());
            format!("{}-{}-{}", year, month, day)
        };
        let folder_path = std::env::current_dir().expect("Can not access current directory")
            .join("uploads").join(&folder);
        if !tokio::fs::try_exists(&folder_path).await.context("Can not access folder")? {
            tokio::fs::create_dir(&folder_path).await.context("Can not create folder")?;
        }
        let file_path = folder_path.join(&file_name);
        let mut file = tokio::fs::File::create(&file_path).await.context("Can not create file")?;
        file.write_all(&data).await.context("Can not write data to file")?;

        sqlx::query!(
            r#"
            INSERT INTO upload (
                upload_id,
                file_name,
                extension,
                content_type,
                folder,
                size
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )
            "#,
            upload_id,
            original_file_name,
            extension,
            content_type,
            folder,
            size
        )
        .execute(&state.pool)
        .await?;

        log::info!(
            "
            Uploading file id {upload_id}
            With name {original_file_name}
            in folder {folder}
            extension {extension}
            content type {content_type}
            and size of {size} bytes"
        );
    }
    Ok(())
}