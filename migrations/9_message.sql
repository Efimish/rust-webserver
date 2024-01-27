create table message
(
    message_id uuid primary key default uuid_generate_v4(),
    chat_id uuid not null references chat (chat_id) on delete cascade,
    sender_id uuid not null references "user" (user_id) on delete cascade,
    reply_message_id uuid references message (message_id),
    forward_message_id uuid references message (message_id),
    context text,
    edited boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    check(message_id != reply_message_id and message_id != forward_message_id)
);