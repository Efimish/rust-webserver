use crate::http::{HttpError, HttpResult};
use anyhow::Context;
use sqlx::PgPool;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

fn folder_name() -> String {
    let today = time::OffsetDateTime::now_utc();
    let year = today.year();
    let month = format!("{:02}", today.month());
    let day = format!("{:02}", today.day());
    format!("{year}-{month}-{day}")
}

fn extension(file_name: &str) -> String {
    use std::path::Path;
    use std::ffi::OsStr;
    Path::new(file_name)
    .extension()
    .and_then(OsStr::to_str)
    .map(|ext| format!(".{ext}"))
    .unwrap_or_default()
}

pub async fn upload_file(
    pool: &PgPool,
    file_name: String,
    content_type: String,
    data: &[u8],
) -> HttpResult<Uuid> {
    let upload_id = Uuid::new_v4();
    let extension = extension(&file_name);
    let new_file_name = format!("{upload_id}{extension}");
    let folder = folder_name();
    let folder_path = std::env::current_dir().expect("Can not access current directory")
        .join("uploads")
        .join(&folder);
    if !tokio::fs::try_exists(&folder_path).await.context("Can not access folder")? {
        tokio::fs::create_dir(&folder_path).await.context("Can not create folder")?;
    }
    let file_path = folder_path.join(&new_file_name);
    let size = data.len() as i64;
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
        file_name,
        extension,
        content_type,
        folder,
        size
    )
    .execute(pool)
    .await?;

    log::debug!("New file uploaded. Id: {upload_id}, name: {file_name}, size: {size}");
    Ok(upload_id)
}

pub async fn upload_avatar(
    pool: &PgPool,
    file_name: String,
    content_type: String,
    data: &[u8],
) -> HttpResult<Uuid> {
    use image::GenericImageView;
    use image::ImageOutputFormat;
    use std::io::Cursor;
    if content_type != "image/jpeg"
    && content_type != "image/png"
    && content_type != "image/webp" {
        return Err(HttpError::BadRequest);
    }
    let mut image = image::load_from_memory(&data)
        .context("Can not load image data")?;
    let (width, height) = image.dimensions();
    let side = width.min(height).min(2048);
    image = image.resize_to_fill(side, side, image::imageops::Nearest);
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::WebP).context("Can not save image to buffer")?;
    let file_name = format!("{}{}", file_name.rsplit_once('.').unwrap().0, ".webp");
    let avatar_id = upload_file(pool, file_name, "image/webp".to_string(), &bytes).await?;
    Ok(avatar_id)
}