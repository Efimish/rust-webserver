use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::http::{HttpResult, HttpError, HttpContext};
use sqlx::PgPool;

use super::{
    track::TrackDTO,
    album::AlbumDTO
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistDTO {
    pub id: Uuid,
    pub name: String,
    pub image: Option<Uuid>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub image: Option<Uuid>,
    pub albums: Vec<AlbumDTO>,
    pub tracks: Vec<TrackDTO>
}

impl ArtistDTO {
    pub async fn upgrade(
        self,
        pool: &PgPool
    ) -> HttpResult<Artist> {
        let albums = sqlx::query_as!(
            AlbumDTO,
            r#"
            SELECT a.* FROM album a
            JOIN artist_album aa
            ON aa.album_id = a.id
            WHERE aa.artist_id = $1
            ORDER BY a.release_date DESC
            "#, self.id
        )
        .fetch_all(pool)
        .await?;

        let tracks = sqlx::query_as!(
            TrackDTO,
            r#"
            SELECT t.* FROM track t
            JOIN artist_track at
            ON at.track_id = t.id
            WHERE at.artist_id = $1
            "#, self.id
        )
        .fetch_all(pool)
        .await?;

        let artist = Artist {
            id: self.id,
            name: self.name,
            image: self.image,
            albums,
            tracks
        };

        Ok(artist)
    }
}

impl Artist {
    pub async fn get(
        pool: &PgPool,
        id: Uuid
    ) -> HttpResult<Self> {
        let artist_dto = sqlx::query_as!(
            ArtistDTO,
            r#"
            SELECT * FROM artist
            WHERE id = $1
            "#, id
        )
        .fetch_optional(pool)
        .await?
        .http_context(HttpError::NotFound)?;

        Ok(artist_dto.upgrade(pool).await?)
    }

    pub async fn get_all(
        pool: &PgPool
    ) -> HttpResult<Vec<Self>> {
        let artists_dto = sqlx::query_as!(
            ArtistDTO,
            r#"
            SELECT * FROM artist
            ORDER BY name
            "#,
        )
        .fetch_all(pool)
        .await?;

        let mut artists = Vec::new();
        for artist_dto in artists_dto {
            artists.push(artist_dto.upgrade(pool).await?);
        }

        Ok(artists)
    }
}