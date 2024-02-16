use std::sync::Arc;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use uuid::Uuid;

use crate::http::{AppState, HttpResult, HttpError, HttpContext};
use crate::http::models::track::Track;

pub async fn get_all_tracks(
    Extension(state): Extension<Arc<AppState>>
) -> HttpResult<Json<Vec<Track>>> {
    let tracks = Track::get_all(&state.pool).await?;
    Ok(Json(tracks))
}

pub async fn get_track(
    Extension(state): Extension<Arc<AppState>>,
    Path(track_id): Path<Uuid>
) -> HttpResult<Json<Track>> {
    let track = Track::get(&state.pool, track_id).await?;
    Ok(Json(track))
}

pub async fn _stream_track(
    Extension(state): Extension<Arc<AppState>>,
    Path(track_id): Path<Uuid>
) -> HttpResult<impl IntoResponse> {
    let track_audio_id = sqlx::query!(
        r#"
        SELECT audio FROM track
        WHERE id = $1
        "#,
        track_id
    )
    .fetch_optional(&state.pool)
    .await?
    .map(|r| r.audio)
    .flatten()
    .http_context(HttpError::NotFound)?;

    Ok(Json(track_audio_id))
}