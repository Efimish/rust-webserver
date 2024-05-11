create table "user"
(
    "id" uuid primary key default gen_random_uuid(),
    "username" text unique not null,
    "email" text unique not null,
    "password_hash" text not null,
    "display_name" text not null,
    "avatar" uuid references "upload" ("id") on delete set null,
    "status" text not null default '',
    "created_at" timestamptz not null default now(),
    "updated_at" timestamptz not null default now(),
    check("username" = lower("username") and "email" = lower("email"))
);

SELECT trigger_updated_at('"user"');