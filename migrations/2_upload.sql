create table "upload"
(
    "id" uuid primary key default gen_random_uuid(),
    "file_name" text not null,
    "extension" text not null,
    "content_type" text not null,
    "folder" text not null,
    "size" bigint not null,
    "created_at" timestamptz not null default now()
);