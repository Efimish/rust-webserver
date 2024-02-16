create table album_track
(
    album_id uuid not null references album (id) on delete cascade,
    track_id uuid not null references track (id) on delete cascade,
    track_number int not null,
    primary key (album_id, track_id),
    unique (album_id, track_number)
);