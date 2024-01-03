create table user_profile
(
    user_id uuid not null references "user" (user_id) on delete cascade,
    display_name text not null,
    status text,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

SELECT trigger_updated_at('user_profile');