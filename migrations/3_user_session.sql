create table user_session
(
    user_id uuid not null references "user" (user_id) on delete cascade,
    session_id uuid not null,
    user_ip text not null,
    user_agent text not null,
    user_country text not null,
    user_city text not null,
    last_active timestamptz not null default now(),
    unique(session_id)
);