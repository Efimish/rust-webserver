create table user_block_user
(
    from_user_id uuid not null references "user" (user_id) on delete cascade,
    to_user_id uuid not null references "user" (user_id) on delete cascade,
    created_at timestamptz not null default now(),
    primary key (from_user_id, to_user_id),
    check(from_user_id != to_user_id)
);