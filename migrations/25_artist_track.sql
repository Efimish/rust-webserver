create table artist_track
(
    artist_id uuid not null references artist (id) on delete cascade,
    track_id uuid not null references track (id) on delete cascade,
    primary key (artist_id, track_id)
);