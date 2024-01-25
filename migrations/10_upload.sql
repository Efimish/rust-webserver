create table upload
(
    upload_id uuid primary key default uuid_generate_v4(),
    file_name text not null,
    file_path text not null,
    content_type text not null,
    size bigint not null,
    created_at timestamptz not null default now()
);