use axum::{Router, routing::get};
mod artists;
mod albums;
mod tracks;

pub fn router() -> Router {
    Router::new()
        .route(
            "/artists",
            get(artists::get_all_artists)
        )
        .route(
            "/artists/:artist_id",
            get(artists::get_artist)
        )
        .route(
            "/albums",
            get(albums::get_all_albums)
        )
        .route(
            "/albums/:album_id",
            get(albums::get_album)
        )
        .route(
            "/tracks",
            get(tracks::get_all_tracks)
        )
        .route(
            "/tracks/:track_id",
            get(tracks::get_track)
        )
        .route(
            "/tracks/:track_id/stream",
            get(_empty)
        )
        .route(
            "/tracks/:track_id/lyrics",
            get(_empty)
        )
}

async fn _empty() {}