use serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::Date;

use crate::http::{HttpResult, HttpError, HttpContext};
use sqlx::PgPool;

use super::{
    track::TrackDTO,
    artist::ArtistDTO
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumDTO {
    pub id: Uuid,
    pub name: String,
    pub image: Option<Uuid>,
    pub release_date: Date
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: Uuid,
    pub name: String,
    pub image: Option<Uuid>,
    pub release_date: Date,
    pub tracks: Vec<TrackDTO>,
    pub artists: Vec<ArtistDTO>
}

impl AlbumDTO {
    pub async fn upgrade(
        self,
        pool: &PgPool
    ) -> HttpResult<Album> {
        let tracks = sqlx::query_as!(
            TrackDTO,
            r#"
            SELECT t.* FROM track t
            JOIN album_track at
            ON at.track_id = t.id
            WHERE at.album_id = $1
            ORDER BY at.track_number DESC
            "#, self.id
        )
        .fetch_all(pool)
        .await?;

        let artists = sqlx::query_as!(
            ArtistDTO,
            r#"
            SELECT a.* FROM artist a
            JOIN artist_album aa
            ON aa.artist_id = a.id
            WHERE aa.album_id = $1
            "#, self.id
        )
        .fetch_all(pool)
        .await?;

        let album = Album {
            id: self.id,
            name: self.name,
            image: self.image,
            release_date: self.release_date,
            tracks,
            artists
        };

        Ok(album)
    }
}

impl Album {
    pub async fn get(
        pool: &PgPool,
        id: Uuid
    ) -> HttpResult<Self> {
        let album_dto = sqlx::query_as!(
            AlbumDTO,
            r#"
            SELECT * FROM album
            WHERE id = $1
            "#, id
        )
        .fetch_optional(pool)
        .await?
        .http_context(HttpError::NotFound)?;

        Ok(album_dto.upgrade(pool).await?)
    }

    pub async fn get_all(
        pool: &PgPool
    ) -> HttpResult<Vec<Album>> {
        let albums_dto = sqlx::query_as!(
            AlbumDTO,
            r#"
            SELECT * FROM album
            ORDER BY release_date DESC
            "#,
        )
        .fetch_all(pool)
        .await?;

        let mut albums = Vec::new();
        for album_dto in albums_dto {
            albums.push(album_dto.upgrade(pool).await?);
        }

        Ok(albums)
    }
}