use std::sync::Arc;

use axum::extract::Path;
use axum::{Extension, Json};
use uuid::Uuid;

use crate::http::{AppState, HttpResult};
use crate::http::models::artist::Artist;

pub async fn get_all_artists(
    Extension(state): Extension<Arc<AppState>>
) -> HttpResult<Json<Vec<Artist>>> {
    let artists = Artist::get_all(&state.pool).await?;
    Ok(Json(artists))
}

pub async fn get_artist(
    Extension(state): Extension<Arc<AppState>>,
    Path(artist_id): Path<Uuid>
) -> HttpResult<Json<Artist>> {
    let artist = Artist::get(&state.pool, artist_id).await?;
    Ok(Json(artist))
}