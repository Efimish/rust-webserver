create table track
(
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    audio uuid references upload (id) on delete set null,
    lyrics text,
    duration_ms bigint not null
);