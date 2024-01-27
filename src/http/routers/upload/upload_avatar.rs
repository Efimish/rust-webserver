use std::sync::Arc;

use crate::http::{AppState, AuthUser, HttpError, HttpResult};
use anyhow::Context;
use axum::{extract::Multipart, Extension};
use image::GenericImageView;
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn upload_avatar(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    mut multipart: Multipart
) -> HttpResult<()> {
    if let Some(field) = multipart.next_field().await
        .context("Can not get next field")?
    {
        let content_type = field.content_type().context("File is missing content type")?.to_string();
        if content_type != "image/jpeg"
        && content_type != "image/png"
        && content_type != "image/webp" {
            return Err(HttpError::BadRequest);
        }
        let original_file_name = field.file_name().context("File is missing file name")?.to_string();
        let data = field.bytes().await.context("File is missing bytes")?;
        let size = data.len() as i64;

        let mut image = image::load_from_memory(&data)
            .context("Can not load image data")?;
        let (width, height) = image.dimensions();
        let side = width.min(height).min(2048);
        image = image.resize_to_fill(side, side, image::imageops::Nearest);
        let upload_id = Uuid::new_v4();
        let file_name = format!("{}.webp", upload_id);
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
        image.save_with_format(file_path, image::ImageFormat::WebP).context("Can not save image")?;
        // let mut file = tokio::fs::File::create(&file_path).await.context("Can not create file")?;
        // file.write_all(&data).await.context("Can not write data to file")?;

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
            ".webp",
            content_type,
            folder,
            size
        )
        .execute(&state.pool)
        .await?;

        sqlx::query!(
            r#"
            UPDATE "user"
            SET avatar = $1
            WHERE user_id = $2
            "#,
            upload_id,
            user.user_id
        )
        .execute(&state.pool)
        .await?;

        log::info!(
            "
            Uploading file id {upload_id}
            With name {original_file_name}
            in folder {folder}
            extension webp
            content type {content_type}
            and size of {size} bytes"
        );
    }
    Ok(())
}