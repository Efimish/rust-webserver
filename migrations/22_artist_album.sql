create table artist_album
(
    artist_id uuid not null references artist (id) on delete cascade,
    album_id uuid not null references album (id) on delete cascade,
    primary key (artist_id, album_id)
);