create table user_read_message
(
    user_id uuid not null references "user" (user_id) on delete cascade,
    message_id uuid not null references message (message_id) on delete cascade,
    created_at timestamptz not null default now(),
    primary key (user_id, message_id)
);