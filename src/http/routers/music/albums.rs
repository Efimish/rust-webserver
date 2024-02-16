use std::sync::Arc;

use axum::extract::Path;
use axum::{Extension, Json};
use uuid::Uuid;

use crate::http::{AppState, HttpResult};
use crate::http::models::album::Album;

pub async fn get_all_albums(
    Extension(state): Extension<Arc<AppState>>
) -> HttpResult<Json<Vec<Album>>> {
    let albums = Album::get_all(&state.pool).await?;
    Ok(Json(albums))
}

pub async fn get_album(
    Extension(state): Extension<Arc<AppState>>,
    Path(album_id): Path<Uuid>
) -> HttpResult<Json<Album>> {
    let album = Album::get(&state.pool, album_id).await?;
    Ok(Json(album))
}