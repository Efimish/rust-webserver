use std::sync::Arc;

use anyhow::Context;
use axum::{extract::Multipart, Extension, Json};
use uuid::Uuid;

use crate::http::{HttpResult, HttpError, AppState, AuthUser, helpers::uploads};

pub async fn edit_my_avatar(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    mut multipart: Multipart
) -> HttpResult<Json<Uuid>> {
    if let Some(field) = multipart.next_field().await
        .context("Can not get next field")?
    {
        let file_name = field.file_name().context("File is missing file name")?.to_string();
        let content_type = field.content_type().context("File is missing content type")?.to_string();
        let data = field.bytes().await.context("File is missing bytes")?;
        let upload_id = uploads::upload_avatar(&state.pool, file_name, content_type, &data).await?;

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
        Ok(Json(upload_id))
    } else {
        Err(HttpError::BadRequest)
    }
}