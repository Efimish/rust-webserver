create table message
(
    id uuid primary key default uuid_generate_v4(),
    chat_id uuid not null references chat (id) on delete cascade,
    sender_id uuid not null references "user" (id) on delete cascade,
    reply_message_id uuid references message (id),
    forward_message_id uuid references message (id),
    context text,
    edited boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    check(id != reply_message_id and id != forward_message_id)
);