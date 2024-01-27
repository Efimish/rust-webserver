create table "user"
(
    user_id uuid primary key default uuid_generate_v4(),
    username text not null,
    email text not null,
    password_hash text not null,
    display_name text not null,
    avatar uuid references upload (upload_id) on delete set null,
    status text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique(username),
    unique(email),
    check(username = lower(username) and email = lower(email))
);

SELECT trigger_updated_at('"user"');