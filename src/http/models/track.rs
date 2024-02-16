use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::http::{HttpResult, HttpError, HttpContext};
use sqlx::PgPool;

use super::{
    album::AlbumDTO,
    artist::ArtistDTO
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackDTO {
    pub id: Uuid,
    pub name: String,
    pub audio: Option<Uuid>,
    pub duration_ms: i64
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: Uuid,
    pub name: String,
    pub audio: Option<Uuid>,
    pub duration_ms: i64,
    pub albums: Vec<AlbumDTO>,
    pub artists: Vec<ArtistDTO>
}

impl TrackDTO {
    pub async fn upgrade(
        self,
        pool: &PgPool,
    ) -> HttpResult<Track> {
        let albums = sqlx::query_as!(
            AlbumDTO,
            r#"
            SELECT a.* FROM album a
            JOIN album_track at
            ON at.album_id = a.id
            WHERE at.track_id = $1
            "#, self.id
        )
        .fetch_all(pool)
        .await?;

        let artists = sqlx::query_as!(
            ArtistDTO,
            r#"
            SELECT a.* FROM artist a
            JOIN artist_track at
            ON at.artist_id = a.id
            WHERE at.track_id = $1
            "#, self.id
        )
        .fetch_all(pool)
        .await?;

        let track = Track {
            id: self.id,
            name: self.name,
            audio: self.audio,
            duration_ms: self.duration_ms,
            albums,
            artists
        };

        Ok(track)
    }
}

impl Track {
    pub async fn get(
        pool: &PgPool,
        id: Uuid
    ) -> HttpResult<Self> {
        let track_dto = sqlx::query_as!(
            TrackDTO,
            r#"
            SELECT * FROM track
            WHERE id = $1
            "#, id
        )
        .fetch_optional(pool)
        .await?
        .http_context(HttpError::NotFound)?;

        Ok(track_dto.upgrade(pool).await?)
    }

    pub async fn get_all(
        pool: &PgPool
    ) -> HttpResult<Vec<Self>> {
        let tracks_dto = sqlx::query_as!(
            TrackDTO,
            r#"
            SELECT * FROM track
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut tracks = Vec::new();
        for track_dto in tracks_dto {
            tracks.push(track_dto.upgrade(pool).await?);
        }

        Ok(tracks)
    }
}