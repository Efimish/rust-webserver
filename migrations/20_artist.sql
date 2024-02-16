create table artist
(
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    image uuid references upload (id) on delete set null
);